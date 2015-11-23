require 'json'
require 'tmpdir'
require 'active_support/core_ext/string/strip'
require 'active_support/core_ext/string/inflections'
require 'active_support/core_ext/string/indent'
require 'turbo_ruby/compiler/project'

module TurboRuby
  class Compiler
    def initialize(input_tree, output_tree, dev: false)
      @in = input_tree
      @out = output_tree
      @dev = dev

      compile!
    end

    private
    def compile!
      parse_gemspec
      parse_trb_files

      build_project
      write_gemspec
      write_cargo_toml
      write_extconf
      write_glue_file
      write_librs
    end

    def parse_gemspec
      candidates = @in.glob('*.gemspec')

      if candidates.length == 1
        @gemspec = load_gemspec(candidates.first)
      elsif candidates.empty?
        raise 'Expected to find a single gemspec file but none was present'
      else
        raise "Expected to find a single gemspec file but found #{candidates.length} instead: #{candidates.join(', ')}"
      end
    end

    # Stolen from RubyGems
    def load_gemspec(gemspec_path)
      begin
        spec = eval(@in.read(gemspec_path), binding, gemspec_path)
      rescue SyntaxError, RuntimeError, Exception => e
        raise "#{gemspec_path} is not a valid gemspec: #{e.message}"
      end

      if Gem::Specification === spec
        spec.loaded_from = gemspec_path
        return spec
      else
        raise "#{gemspec_path} is not a valid gemspec"
      end
    end

    def parse_trb_files
      parser = Parser.new

      @in.glob('src/**/*.trb').each do |trb|
        parser.parse(@in.read(trb))
      end

      @ast = parser.ast
    end

    def build_project
      @project = Project.new(@gemspec, @ast)
    end

    def write_gemspec
      @out.write("#{@project.name}.gemspec", @in.read(@in.glob('*.gemspec').first))
    end

    def write_cargo_toml
      name = @project.name
      version = @project.version.to_s
      authors = JSON.generate(@project.emails.map { |(author, email)| "#{author} <#{email}>" })

      cargo_toml = <<-TOML.strip_heredoc
        [package]
        name = #{name.inspect}
        version = #{version.inspect}
        authors = #{authors}

        [lib]
        name = #{name.inspect}
        doctest = false
        crate-type = ["staticlib"]

        [dependencies]
        libc = "=0.2.1"
      TOML

      if @dev
        cargo_toml.concat <<-TOML.strip_heredoc

          [dependencies.ffi_buf]
          version = "1.0.0"
          path = "#{File.expand_path('../../../crates/ffi_buf', __FILE__)}"
        TOML
      else
        cargo_toml << 'ffi_buf = "1.0.0"' << "\n"
      end

      @out.write "Cargo.toml", cargo_toml
    end

    def write_extconf
      name = @project.name

      @out.write "extconf.rb", <<-EXTCONF.strip_heredoc
        require "mkmf"

        LIB_DIRS = [
          File.expand_path("../target/release", __FILE__)
        ]

        dir_config(#{name.inspect}, nil, LIB_DIRS)

        $LOCAL_LIBS << #{('-l' + name).inspect}

        create_makefile #{name.inspect}
      EXTCONF
    end

    def write_glue_file
      name = @project.name

      glue_file = <<-GLUE_FILE.strip_heredoc
        #include <ruby.h>
        #include <ruby/encoding.h>

        typedef struct {
          void* ptr;
          size_t len;
        } trb_buf_t;

        static inline trb_buf_t
        STR2BUF(VALUE str) {
          return (trb_buf_t) {
            .ptr = RSTRING_PTR(str),
            .len = RSTRING_LEN(str),
          };
        }

      GLUE_FILE

      init_blocks = []

      @ast.each do |klass|
        klass.methods.each do |method|
          glue_file << prototype_for(klass, method) << "\n"
          glue_file << c_method_for(klass, method) << "\n"
          init_blocks << init_block_for(klass, method)
        end
      end

      glue_file << "void Init_#{name}( void ) {" << "\n"
      glue_file << init_blocks.join("\n").indent(2) << "\n"
      glue_file << "}" << "\n"

      @out.write "#{name}.c", glue_file
    end

    def write_librs
      librs = <<-LIBRS.strip_heredoc
        extern crate libc;
        extern crate ffi_buf;

        use ffi_buf::Buf;

      LIBRS

      @ast.each do |klass|
        librs << "trait #{trait_name(klass)} {" << "\n"
        librs << klass.methods.map { |m| trait_method(m) + ";" }.join("\n").indent(4) << "\n"
        librs << "}" << "\n" << "\n"
      end

      @ast.each do |klass|
        librs << "impl<'a> #{trait_name(klass)} for #{native_rust_type(klass)} {" << "\n"

        klass.methods.each do |method|
          librs << "#{trait_method(method)} {".indent(4) << "\n"
          librs << method.body.join.strip_heredoc.indent(8)
          librs << "}".indent(4) << "\n"
        end

        librs << "}" << "\n" << "\n"
      end

      @ast.each do |klass|
        klass.methods.each do |method|
          librs << "#[no_mangle]" << "\n"
          librs << "pub extern \"C\" fn trb_#{c_method_name(klass, method)}" \
                   "(rb_self: #{rust_transport_type(klass)})"
          librs << " -> #{method.return_type}" if method.return_type
          librs << " {" << "\n"
          librs << "#{transport_to_native("rb_self", klass)}.trb_#{method_name(method)}()".indent(4) << "\n"
          librs << "}" << "\n"
        end
      end

      @out.write "src/lib.rs", librs
    end

    def prototype_for(klass, method)
      return_type = 'int'
      self_type = 'trb_buf_t'
      "#{return_type} trb_#{c_method_name(klass, method)}(#{self_type} self);\n"
    end

    def c_method_for(klass, method)
      name = c_method_name(klass, method)
      expr = "trb_#{name}(#{ruby_to_rust(klass.name, 'self')})"
      ret = rust_to_ruby(method.return_type, expr)

      <<-METHOD.strip_heredoc
        static VALUE
        #{name}(VALUE self) {
          return #{ret};
        }
      METHOD
    end

    def init_block_for(klass, method)
      name = c_method_name(klass, method)
      "rb_define_method(#{c_class_for(klass)}, #{method.name.inspect}, #{name}, #{method.arguments.length});"
    end

    def c_method_name(klass, method)
      klass_name = klass.name.underscore
      "#{klass_name}_#{method_name(method)}"
    end

    def method_name(method)
      method.name.gsub(/\A(.*)\?\z/, 'is_\1')
    end

    def ruby_to_rust(ruby_type, expr)
      "STR2BUF(#{expr})"
    end

    def rust_to_ruby(rust_type, expr)
      "(#{expr} ? Qtrue : Qfalse)"
    end

    def c_class_for(klass)
      "rb_cString"
    end

    def trait_name(klass)
      "Ruby#{klass.name}"
    end

    def trait_method(method)
      if method.return_type
        "fn trb_#{method_name(method)}(&self) -> #{method.return_type}"
      else
        "fn trb_#{method_name(method)}(&self)"
      end
    end

    def native_rust_type(klass)
      "&'a str"
    end

    def rust_transport_type(klass)
      "Buf"
    end

    def transport_to_native(param_name, klass)
      "#{param_name}.as_slice()"
    end
  end
end



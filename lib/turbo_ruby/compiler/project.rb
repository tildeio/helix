require 'active_support/core_ext/string/strip'
require 'active_support/core_ext/string/inflections'
require 'active_support/core_ext/string/indent'

module TurboRuby
  class Compiler
    class Project
      def initialize(gemspec, ast)
        @gemspec = gemspec
        @classes = ast.map { |a| Class.new(self, a) }
      end

      def name
        @gemspec.name
      end

      def version
        @gemspec.version
      end

      def authors
        @gemspec.authors
      end

      def emails
        @gemspec.authors.zip(@gemspec.email)
      end
    end

    class Class
      def initialize(project, ast)
        @type = Type.from_ruby(ast.name)
        @project = project
        @methods = ast.methods.map { |m| Method.new(m, klass_name: ast.name, project_prefix: project.name) }
      end
    end

    class Method
      def initialize(node, klass_name:, project_prefix: "trb")
        @name = node.name
        @klass = Type.from_ruby(klass_name)
        @project_prefix = project_prefix
        @return_type = Type.from_rust(node.return_type)
        @arguments = Arguments.new(@klass, node.arguments.map { |a| Argument.new(a) })
        @body = node.body
      end

      def init_method
        <<-C.strip_heredoc.chomp
          rb_define_method(#{@project_prefix}_#{@klass.ruby_name}, "#{@name}", #{c_name}, #{@arguments.arity});
        C
      end

      def static_function
        <<-C.strip_heredoc.chomp
          static VALUE
          #{c_name}(VALUE self) {
            bool ret;
            CHECK_FFI(#{@project_prefix}_#{c_name}(#{@arguments.to_rust_args}, &ret));
            return BOOL2RB(ret);
          }
        C
      end

      def c_rust_prototype
        "boolean #{@project_prefix}_#{c_name}(#{@arguments.to_prototype_params}, boolean*);"
      end

      def rust_extern_fn
        <<-RUST.strip_heredoc.chomp
          #[no_mangle]
          pub extern "C" fn #{global_c_name}(#{@arguments.to_extern_params}, out: *mut bool) -> Option<ErrorDesc> {
            unsafe { *ret = #{@arguments.self_to_rust_native}.#{@project_prefix}_#{portable_name}(#{@arguments.args_to_rust_native}) };
            None
          }
        RUST
      end

      def rust_trait_impl
        body = @body.join.strip_heredoc.indent(4).chomp
        <<-RUST.strip_heredoc.chomp
          #{rust_trait_header} {
          #{body}
          }
        RUST
      end

      def rust_trait_def
        "#{rust_trait_header};"
      end

      private
      def portable_name
        @name.sub(/\?/, '_p')
      end

      def c_name
        "#{@klass.prefix}_#{portable_name}"
      end

      def global_c_name
        "#{@project_prefix}_#{c_name}"
      end

      def rust_trait_header
        "fn #{@project_prefix}_#{portable_name}(&self) -> #{@return_type.native_rust}"
      end
    end

    class Arguments < Struct.new(:self, :list)
      def arity
        self.list.length
      end

      def to_rust_args
        self.self.to_transport('self')
      end

      def to_prototype_params
        "#{self.self.c_transport_type} self"
      end

      def to_extern_params
        "rb_self: #{self.self.rust_transport_type}"
      end

      def self_to_rust_native
        self.self.rust_trait_param('rb_self')
      end

      def args_to_rust_native
        ""
      end
    end

    class Argument
      def initialize(ast)
        @name = ast.name
        @type = Type.from_rust(ast.type)
      end
    end

    class Type < Struct.new(:rust_name, :ruby_name)
      def self.from_rust(name)
        case name
        when 'bool'
          Boolean.new
        else
          Type.new("String", "String")
        end
      end

      def self.from_ruby(name)
        Type.new("String", "String")
      end

      def prefix
        self.ruby_name.underscore
      end

      def to_transport(name)
        "STR2BUF(#{name})"
      end

      def rust_trait_param(name)
        "#{name}.as_slice()"
      end

      def c_transport_type
        "trb_buf_t"
      end

      def rust_transport_type
        "Buf"
      end

      class Boolean < Type
        def native_rust
          "bool"
        end
      end
    end
  end
end

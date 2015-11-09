require 'json'
require 'tmpdir'

module TurboRuby
  class Compiler
    def initialize(input_tree, output_tree)
      @in = input_tree
      @out = output_tree

      compile!
    end

    private
    def compile!
      parse_gemspec
      parse_trb_files

      write_gemspec
      write_cargo_toml
      write_extconf
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

    def write_gemspec
      @out.write("#{@gemspec.name}.gemspec", @in.read(@in.glob('*.gemspec').first))
    end

    def write_cargo_toml
      name = @gemspec.name
      version = @gemspec.version.to_s
      authors = JSON.generate(@gemspec.authors.zip(@gemspec.email).map { |(author, email)| "#{author} <#{email}>" })

      @out.write "Cargo.toml", <<TOML
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
    end

    def write_extconf
      name = @gemspec.name

      @out.write "extconf.rb", <<EXTCONF
require "mkmf"

LIB_DIRS = [
  File.expand_path("../target/release", __FILE__)
]

dir_config(#{name.inspect}, nil, LIB_DIRS)

$LOCAL_LIBS << #{('-l' + name).inspect}

create_makefile #{name.inspect}
EXTCONF
    end
  end
end

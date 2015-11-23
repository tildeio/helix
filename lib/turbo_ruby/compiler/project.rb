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
        @type = RubyType.new(ast.name)
        @project = project
        @methods = ast.methods.map { |m| Method.new(project, self, m) }
      end
    end

    class Method
      def initialize(project, klass, ast)
        @name = ast.name
        @project, @klass = project, klass
        @return_type = RustType.new(ast.return_type)
        @arguments = ast.arguments.map { |a| Argument.new(a) }
        @body = ast.body
      end
    end

    class Argument
      def initialize(ast)
        @name = ast.name
        @type = RustNativeType.new(ast.type)
      end
    end

    class RustType < Struct.new(:name)
    end

    class RustTransportType < Struct.new(:name)

    end

    class RustNativeType < Struct.new(:name)
      def to_transport
        "Buf"
      end
    end

    class RubyType < Struct.new(:name)
    end
  end
end

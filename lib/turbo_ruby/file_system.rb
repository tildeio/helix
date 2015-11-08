require 'fileutils'
require 'stringio'

module TurboRuby
  module FileSystem
    class Tree
      attr_reader :root
      protected :root

      def initialize(root)
        @root = File.expand_path(root)
      end

      def file?(path)
        File.file?(expand_path(path))
      end

      def directory?(path)
        File.directory?(expand_path(path))
      end

      def read(path)
        path = expand_path(path)
        File.read(expand_path(path))
      end

      def write(path, content = '', &block)
        mkdir_p(File.dirname(path))
        path = expand_path(path)

        if block_given?
          File.open(path, 'w+', &block)
        else
          File.write(path, content)
        end
      end

      def mkdir_p(path)
        FileUtils.mkdir_p(expand_path(path))
      end

      def glob(pattern, &block)
        Dir.chdir(@root) { Dir.glob(pattern, &block) }
      end

      def ==(other)
        case other
        when Tree
          @root == other.root
        when InMemoryTree
          other == self
        when Hash
          InMemoryTree.new(other) == self
        else
          false
        end
      end

      private
      def expand_path(path)
        File.expand_path(path, @root)
      end
    end

    class InMemoryTree
      include Enumerable

      def self.from_root(root)
        from_tree(Tree.new(root))
      end

      def self.from_tree(fs_tree)
        InMemoryTree.new.tap do |mem_tree|
          fs_tree.glob('**/*') do |filename|
            if fs_tree.directory?(filename)
              mem_tree.mkdir_p(filename)
            else
              mem_tree.write(filename, fs_tree.read(filename))
            end
          end
        end
      end

      def initialize(tree = {})
        @tree = tree
      end

      def each(&block)
        walk_tree(@tree, &block)
      end

      def file?(path)
        get(path).is_a?(String) rescue false
      end

      def directory?(path)
        get(path).is_a?(Hash) rescue false
      end

      def read(path)
        if file?(path)
          get(path)
        elsif directory?(path)
          raise Errno::EISDIR, 'Is a directory'
        else
          raise Errno::ENOENT, "No such file or directory - #{path}"
        end
      end

      def write(path, content = '', &block)
        raise Errno::EISDIR, 'Is a directory' if directory?(path)

        parts = path.split('/').map(&:to_sym)
        filename = parts.pop
        dir = nil

        if parts.empty?
          dir = @tree
        else
          dirname = parts.join('/')
          mkdir_p(dirname)
          dir = get(dirname)
        end

        if block_given?
          yield StringIO.new(dir[filename] = '')
        else
          dir[filename] = content
        end
      end

      def mkdir_p(path)
        dirs = path.split('/').map(&:to_sym)
        dirs.inject(@tree) { |cwd, child| cwd[child] ||= {} }
      end

      def glob(pattern, &block)
        map { |name, _| name }
          .find_all { |name| File.fnmatch(pattern, name, File::FNM_PATHNAME) }
          .tap { |list| list.each(&block) if block_given? }
      end

      def to_h
        @tree
      end

      def ==(other)
        case other
        when Hash
          @tree == other
        when InMemoryTree
          @tree == other.to_h
        when Tree
          glob('**/*').sort == other.glob('**/*').sort &&
            all? do |filename, content|
              case content
              when Hash
                other.directory?(filename)
              else
                other.read(filename) == content
              end
            end
        else
          false
        end
      end

      private
      def get(path)
        parts = path.split('/').map(&:to_sym)
        target = parts.pop
        cwd = @tree

        while child = parts.shift
          case cwd = cwd[child]
          when Hash
            # Carry on
          when String, nil
            raise Errno::ENOENT, "No such file or directory - #{path}"
          end
        end

        cwd[target]
      end

      def walk_tree(path = [], tree, &block)
        tree.each_pair do |name, node|
          current_path = path + [name]

          block.call(current_path.join('/'), node)

          if node.is_a?(Hash)
            walk_tree(current_path, node, &block)
          end
        end
      end
    end
  end
end
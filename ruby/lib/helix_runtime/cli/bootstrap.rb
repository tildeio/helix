require 'thor'
require 'bundler/cli'
require 'bundler/cli/gem'

module HelixRuntime
  module CLI
    class Bootstrap < Thor::Group
      argument :path, type: :string
      argument :name, type: :string, optional: true

      include Thor::Actions

      desc "Bootstrap Helix"
      source_root File.expand_path("../templates", __FILE__)

      # NOTE: Instead of using destination_root, we include the full path so
      #   that we see the path relative to the root of where we're running the command.

      def build_gem
        # Add Bundler templates to path
        self.class.source_paths << Bundler::CLI.source_root

        # Set shell output
        Bundler.ui = Bundler::UI::Shell.new

        gem_cli = Bundler::CLI::Gem.new({}, app_name, self)
        # Set custom target
        gem_cli.instance_variable_set(:@target, Pathname.new(base_path))
        # No-op the safe check, so we can re-run the command.
        # We may later want to have a separate update command instead of doing this.
        def gem_cli.ensure_safe_gem_name(*); end

        # Run
        gem_cli.run
      ensure
        # Remove bundler templates from path
        self.class.source_paths.delete(Bundler::CLI.source_root)
      end

      def create_cargo_toml
        template "Cargo.toml", "#{base_path}/Cargo.toml"
      end

      def add_rust_lib_file
        template "lib.rs", "#{base_path}/src/lib.rs"
      end

      def create_gemspec
        # template "gem.gemspec", "#{base_path}/#{app_name}.gemspec"
        insert_into_file "#{base_path}/#{app_name}.gemspec",
            "\n\n  spec.add_dependency 'helix_runtime', '~> #{HelixRuntime::GEM_VERSION}'",
            after: 'spec.require_paths = ["lib"]'
      end

      def add_rake_task
        template "helix_runtime.rake", "#{base_path}/lib/tasks/helix_runtime.rake"
      end

      def add_ruby_lib_file
        append_to_file "#{base_path}/lib/#{app_name}.rb", <<-FILE

require "helix_runtime"
begin
  require "#{app_name}>/native"
rescue LoadError
  warn "Unable to load #{app_name}/native. Please run `rake build`"
end
FILE
      end

      def update_gitignore
        append_to_file "#{base_path}/.gitignore", <<-FILE
/target/
*.bundle
*.so
        FILE
      end

      def update_rakefile
        append_to_file "#{base_path}/Rakefile", "import 'lib/tasks/helix_runtime.rake'\n"
      end

      def update_git
        if Bundler.git_present?
          `git add .`
        end
      end

      private

        def base_path
          File.expand_path(path)
        end

        def app_name
          name || File.basename(base_path)
        end

        def class_name
          app_name.split("_").map(&:capitalize).join
        end
    end
  end
end

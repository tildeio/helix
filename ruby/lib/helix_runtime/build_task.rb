require 'rake/tasklib'
require 'helix_runtime'

module HelixRuntime
  class BuildTask < Rake::TaskLib

    def self.delegate_attr(getter, to:)
      setter = "#{getter}="
      define_method(getter,       -> { send(to).send(getter) })
      define_method(setter, -> (val) { send(to).send(setter, val) })
    end

    def project
      @project ||= Project.new(Dir.pwd)
    end

    delegate_attr :helix_lib_dir,  to: :project
    delegate_attr :debug_rust,     to: :project
    delegate_attr :build_root,     to: :project

    attr_accessor :pre_build

    def initialize(deprecated_name = nil)
      yield self if block_given?

      if deprecated_name
        warn "DEPRECATION WARNING: Passing a project name to the Helix build " \
          "task (`HelixRuntime::BuildTask.new(#{deprecated_name.inspect})`) " \
          "is unnecessary, as we now automatically detect the project name " \
          "from your `Cargo.toml`.\n\n"
      end

      define
    end

    def define
      task "helix:pre_build" do
        pre_build.call if pre_build
      end

      task "helix:check_path" do
        begin
          HelixRuntime.ensure_dll!
        rescue HelixRuntime::MissingDllError => e
          puts e.message
          abort "Run `rake helix:copy_dll` to copy to your Ruby bin dir."
        end
      end

      task "helix:copy_dll" do
        HelixRuntime.copy_dll
      end

      task "cargo:build" => ["helix:pre_build", "helix:check_path"] do
        project.cargo_build || abort
      end

      task "cargo:clean" do
        project.cargo_clean
      end

      desc "Build #{project.name}"
      task :build => ["helix:pre_build", "helix:check_path"] do
        project.build || abort
      end

      desc "Remove build artifacts"
      task :clobber do
        project.clobber
      end

      desc "Launch an IRB console for #{project.name}"
      task :irb => :build do
        exec "bundle exec irb -r#{project.name} -Ilib"
      end
    end

  end
end

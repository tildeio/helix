require 'rake/tasklib'
require 'rake/clean'
require 'helix_runtime/platform'

module HelixRuntime
  class BuildTask < Rake::TaskLib

    IS_WINDOWS = RUBY_PLATFORM =~ /mingw/

    attr_accessor :name
    attr_accessor :build_path
    attr_accessor :lib_path

    def initialize(name = nil, gem_spec = nil)
      init(name, gem_spec)
      yield self if block_given?
      define
    end

    def init(name = nil, gem_spec = nil)
      @name = name
      @build_path = "target/release"
      @lib_path = "lib/#{name}"
    end

    def define
      fail "Extension name must be provided." if @name.nil?
      @name = @name.to_s

      task "helix:check_path" do
        if IS_WINDOWS
          dll = "helix-runtime-#{VERSION.gsub('.', '-')}.#{Platform.libext}"
          found = ENV['PATH'].split(';').any? do |dir|
            File.exist?(File.expand_path("#{dir}/#{dll}", __FILE__))
          end

          unless found
            abort "Unable to find #{dll} in $PATH.\n" \
                  "Please copy #{File.expand_path("../native.so", __FILE__)} to #{dll} at a location in your $PATH"
          end
        else
          # No-op
        end
      end

      # Checking the path isn't a real dependency, but this is a good time to do it
      task "#{name}:cargo:build" => "helix:check_path" do
        extra_args = IS_WINDOWS ? "" : " -- -C link-args='-Wl,-undefined,dynamic_lookup'"

        sh "cargo rustc --release #{extra_args}"
      end

      task "#{name}:cargo:clean" do
        sh "cargo clean"
      end

      directory lib_path

      task "clobber" => "#{name}:cargo:clean"

      libfile_prefix = IS_WINDOWS ? '' : 'lib'
      native_path = "#{lib_path}/native.#{Platform.dlext}"
      native_lib = "#{libfile_prefix}#{name}.#{Platform.libext}"

      file native_path => [lib_path, "#{name}:cargo:build"] do
        cp "#{build_path}/#{native_lib}", native_path
      end
      CLOBBER.include(native_path)

      desc "Build Helix Library for #{name}"
      task "#{name}:build" => native_path

      task "#{name}:irb" => native_path do
        exec "irb -lib -r#{name}"
      end
    end

  end
end
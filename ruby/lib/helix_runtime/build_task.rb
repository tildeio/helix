require 'rake/tasklib'
require 'rake/clean'
require 'helix_runtime/version'
require 'helix_runtime/platform'

module HelixRuntime
  class BuildTask < Rake::TaskLib

    IS_WINDOWS = RUBY_PLATFORM =~ /mingw/

    attr_accessor :name
    attr_accessor :build_path
    attr_accessor :lib_path
    attr_accessor :helix_lib_dir

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
          unless dll_path
            abort "Unable to find #{dll_filename} in $PATH.\n" \
                  "Run `rake helix:copy_dll` to copy to your Ruby bin dir or manually copy #{File.expand_path("../native.so", __FILE__)} to #{dll_filename} at a location in your $PATH"
          end
        else
          # No-op
        end
      end

      task "helix:copy_dll" do
        if IS_WINDOWS
          so_path = File.expand_path("../native.#{Platform.dlext}", __FILE__)
          abort "Unable to find native bundle at #{so_path}" unless File.exists?(so_path)

          bindir = RbConfig::CONFIG['bindir']
          abort "Unable to determine Ruby bindir" unless bindir

          cp so_path, File.join(bindir, dll_filename)
        else
          # No-op
        end
      end

      # Checking the path isn't a real dependency, but this is a good time to do it
      task "#{name}:cargo:build" => "helix:check_path" do
        # We have to do this here since Cargo has no internal means of specifying `-C` flags
        link_args = if IS_WINDOWS
          # SAFESEH is added to i686 Rust hosts
          # https://github.com/rust-lang/rust/blob/1.15.1/src/librustc_back/target/i686_pc_windows_msvc.rs#L25
          if `rustc -vV` =~ /host:\s+i686/
            '/SAFESEH:NO' # Can't use SAFESEH with .libs from dlltool
          end
        else
          # Allowing all methods to be undefined is a bit risky, would be nice to have a specific list.
          '-Wl,-undefined,dynamic_lookup'
        end

        env = {}
        env['HELIX_LIB_DIR'] = helix_lib_dir if helix_lib_dir

        extra_args = link_args ? " -- -C link-args=#{link_args}" : ''

        sh env, "cargo rustc --release#{extra_args}"
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
        exec "bundle exec irb -r#{name}"
      end
    end

    private

    def dll_filename
      @dll_filename ||= "helix-runtime-#{VERSION.gsub('.', '-')}.#{Platform.libext}"
    end

    def dll_path
      return nil unless IS_WINDOWS
      return @dll_path if @dll_path_searched

      dir = ENV['PATH'].split(';').find do |dir|
        File.exist?(File.expand_path("#{dir}/#{dll_filename}", __FILE__))
      end

      @dll_path_searched = true
      @dll_path = dir ? File.join(dir, dll_filename) : nil
    end

  end
end

require 'rake/clean'
require_relative 'platform'

is_windows = RUBY_PLATFORM =~ /mingw/

directory "lib/#{$example_name}"

namespace :cargo do
  task :build do
    if is_windows
      sh "cargo rustc --release"
    else
      sh "cargo rustc --release -- -C link-args='-Wl,-undefined,dynamic_lookup'"
    end
  end

  task :clean do
    sh "cargo clean"
  end
end

task :clobber => "cargo:clean"

libfile_prefix = is_windows ? '' : 'lib'
example_lib_path = "lib/#{$example_name}"

if is_windows
  # Copy DLLs
  native_so = "../../ruby/lib/helix_runtime/native.so"

  file "#{example_lib_path}/native.dll" => native_so do
    cp native_so, "#{example_lib_path}/native.dll"
  end
  CLOBBER.include("#{example_lib_path}/native.dll")

  ruby_dll = nil
  ENV['PATH'].split(/;/).each do |dir|
    # FIXME: Don't hardcode Ruby version if possible
    path = "#{dir}/msvcrt-ruby230.dll"
    if File.exist?(path)
      ruby_dll = path
      break
    end
  end

  abort "Couldn't find ruby dll in PATH'" unless ruby_dll

  file "#{example_lib_path}/ruby.dll" => ruby_dll do
    cp ruby_dll, "#{example_lib_path}/ruby.dll"
  end
  CLOBBER.include("#{example_lib_path}/ruby.dll")

  task :build_native => ["#{example_lib_path}/native.dll", "#{example_lib_path}/ruby.dll"]
end

example_native_path = "#{example_lib_path}/native.#{Platform.dlext}"
example_native_lib = "#{libfile_prefix}#{$example_name}.#{Platform.libext}"

file example_native_path => [example_lib_path, "cargo:build"] do
  cp "../../target/release/#{example_native_lib}", example_native_path
end
CLOBBER.include(example_native_path)

task :irb => example_native_path do
  exec "irb -Ilib -r#{$example_name}"
end

task :build_native => example_native_path

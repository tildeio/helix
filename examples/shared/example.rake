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

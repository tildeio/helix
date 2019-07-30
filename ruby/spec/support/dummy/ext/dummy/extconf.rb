root_dir = File.expand_path("../../../../../..", __FILE__)

require "mkmf"
require "#{root_dir}/lib/helix_runtime/version"

dir_config "dummy"
find_header "helix_runtime.h", "#{root_dir}/ext/helix_runtime/native"

if RUBY_PLATFORM =~ /mingw/
  append_ldflags("-L#{root_dir}/windows_build -lhelix-runtime-#{HelixRuntime::VERSION.gsub('.', '-')}")
end


create_makefile "dummy"

require "mkmf"

root_dir = File.expand_path("../../../../../..", __FILE__)

dir_config "dummy"
find_header "helix_runtime.h", "#{root_dir}/ext/helix_runtime/native"

if RUBY_PLATFORM =~ /mingw/
  # Is this the correct place to run these commands?
  FileUtils.mkdir_p("#{root_dir}/windows_build")

  res = system("lib -def:#{root_dir}/ext/helix_runtime/native/native.def -out:#{root_dir}/windows_build/helix_runtime.lib")
  abort "can't build helix_runtime.lib" unless res
  append_ldflags("-L#{root_dir}/windows_build -lhelix_runtime")

  # Windows looks for a .dll in PATH
  FileUtils.cp "#{root_dir}/lib/helix_runtime/native.so", "#{root_dir}/windows_build/native.dll"
end

create_makefile "dummy"

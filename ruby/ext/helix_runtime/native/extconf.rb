require "mkmf"

if RUBY_PLATFORM =~ /mingw/
  system "rake native_def_file"
end

create_makefile "helix_runtime/native"

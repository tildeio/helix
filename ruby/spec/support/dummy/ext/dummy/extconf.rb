require "mkmf"
dir_config "dummy"
find_header "helix_runtime.h", File.expand_path("../../../../../../ext/helix_runtime/native", __FILE__)
create_makefile "dummy"

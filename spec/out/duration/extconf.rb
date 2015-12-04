require "mkmf"

LIB_DIRS = [
  File.expand_path("../target/release", __FILE__)
]

dir_config("activesupport_duration", nil, LIB_DIRS)

$LOCAL_LIBS << "-lactivesupport_duration"

create_makefile "native_activesupport_duration"

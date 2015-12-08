require "mkmf"

LIB_DIRS = [
  File.expand_path("../target/release", __FILE__)
]

dir_config("zomg", nil, LIB_DIRS)

$LOCAL_LIBS << "-lzomg"

create_makefile "zomg"

require 'mkmf'

LIB_DIRS = [
  File.expand_path("../target/release", __FILE__)
]

dir_config('fast_blank', nil, LIB_DIRS)

$LOCAL_LIBS << '-lfast_blank'

create_makefile 'fast_blank'


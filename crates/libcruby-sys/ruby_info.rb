require 'rbconfig'

puts RbConfig::CONFIG['libdir'] # C:/Ruby23/lib
puts RbConfig::CONFIG['LIBRUBY'] # libmsvcrt-ruby230.dll.a
puts RbConfig::CONFIG['LIBRUBY_SO'] # msvcrt-ruby230.dll

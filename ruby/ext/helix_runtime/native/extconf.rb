require "mkmf"

if RUBY_PLATFORM =~ /mingw/
  # Hackish way to pull exported methods out of the header
  header_path = File.expand_path("../helix_runtime.h", __FILE__)

  exports = []
  File.readlines(header_path).each do |line|
    if line =~ /^\w.+(HELIX_[\w_]+)[;\(]/
      exports << $1
    end
  end

  File.open(File.expand_path("../native.def", __FILE__), 'w') do |f|
    f.puts "LIBRARY helix-runtime"
    f.puts "EXPORTS"
    f.puts exports
    f.puts "Init_native"
  end
end

create_makefile "helix_runtime/native"

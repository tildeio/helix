require "helix_runtime"

begin
  require "unit/native"
rescue LoadError
  warn "Unable to load unit/native. Please run `rake build`"
end

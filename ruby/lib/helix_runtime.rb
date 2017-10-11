require "helix_runtime/version"

begin
  require "helix_runtime/native"
rescue LoadError
  warn "Unable to load helix_runtime/native. Please run `rake compile` in #{File.expand_path("../..", __FILE__)}."
end

module HelixRuntime
end

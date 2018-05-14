require "helix_runtime"

require "action_dispatch"
require "action_dispatch/journey"

case ENV["IMPLEMENTATION"]
when "RUST"
  require "journey/native"

  ActionDispatch::Journey::Scanner = ::RustJourneyScanner
when "RAILS"
when "RUBY"
  require "journey/ruby"
  ActionDispatch::Journey::Scanner = ::Journey::Ruby::Scanner
when "NONE"
else
  puts "\nPlease specify an IMPLEMENTATION: RUST, RAILS or NONE"
  exit!
end

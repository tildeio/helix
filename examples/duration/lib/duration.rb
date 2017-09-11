require "helix_runtime"

require "active_support"
require "active_support/inflector"
require "active_support/time"
require "active_support/json"

case ENV["IMPLEMENTATION"]
when "RUST"
  require "duration/native"

  ActiveSupport::Duration = ::RustDuration
when "RAILS"
  require "active_support/duration"
when "NONE"
else
  puts "\nPlease specify an IMPLEMENTATION: RUST, RAILS or NONE"
  exit!
end

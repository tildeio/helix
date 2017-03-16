require "helix_runtime"

require "active_support"
require "active_support/inflector"
require "active_support/time"
require "active_support/json"

case ENV["IMPLEMENTATION"]
when "RUST"
  require "duration/native"

  ::Duration.class_eval do
    alias_method :+, :plus
    alias_method :-, :minus
    alias_method :-@, :negate
    alias_method :<=>, :cmp
    alias_method :==, :eq

    # FIXME: We don't handle default arguments in Rust yet
    def iso8601(precision: nil)
      iso8601_precise(precision)
    end
  end

  ActiveSupport::Duration = ::Duration
when "RAILS"
  require "active_support/duration"
when "NONE"
else
  puts "\nPlease specify an IMPLEMENTATION: RUST, RAILS or NONE"
  exit!
end

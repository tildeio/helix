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

    # We have to do this here since it's actually operating on the Time
    def since(time = ::Time.current)
      sum(1, time)
    end
    alias :from_now :since

    # We have to do this here since it's actually operating on the Time
    def ago(time = ::Time.current)
      sum(-1, time)
    end
    alias :until :ago

    # FIXME: We don't handle default arguments in Rust yet
    def iso8601(precision: nil)
      iso8601_precise(precision)
    end

    protected

      def sum(sign, time = ::Time.current) #:nodoc:
        parts.inject(time) do |t,(type,number)|
          if t.acts_like?(:time) || t.acts_like?(:date)
            if type == :seconds
              t.since(sign * number)
            elsif type == :minutes
              t.since(sign * number * 60)
            elsif type == :hours
              t.since(sign * number * 3600)
            else
              t.advance(type => sign * number)
            end
          else
            raise ::ArgumentError, "expected a time or date, got #{time.inspect}"
          end
        end
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

require 'active_support'
require 'active_support/duration'

module ActiveSupport
  remove_const :Duration
end

require 'native_activesupport_duration'
require 'active_support/core_ext/date/calculations'
require 'active_support/core_ext/date_time/calculations'
require 'active_support/core_ext/object/acts_like'

module ActiveSupport
  class << Duration
    private
    def dispatch_advance_seconds(obj, amount)
      obj.advance(seconds: amount)
    end

    def dispatch_advance_minutes(obj, amount)
      obj.advance(minutes: amount)
    end

    def dispatch_advance_hours(obj, amount)
      obj.advance(hours: amount)
    end

    def dispatch_advance_days(obj, amount)
      obj.advance(days: amount)
    end

    def dispatch_advance_months(obj, amount)
      obj.advance(months: amount)
    end

    def dispatch_advance_years(obj, amount)
      obj.advance(years: amount)
    end
  end
end


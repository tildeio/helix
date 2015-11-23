$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
require 'turboruby'
require 'active_support'
require 'minitest'

module TurboRuby
  module TestHelpers
    def assert(val)
      expect(val).to be_truthy
    end
  end
end

RSpec.configure do |c|
  c.include TurboRuby::TestHelpers
end

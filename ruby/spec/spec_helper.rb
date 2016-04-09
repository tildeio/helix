$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)
$LOAD_PATH.unshift File.expand_path('../support/dummy/lib', __FILE__)
require 'helix_runtime'
require 'neversaydie' unless ENV["ALWAYS_SAY_DIE"]
require 'dummy'

module HelixRuntime
  module SpecHelpers
    extend RSpec::Matchers::DSL
    SEGV = NeverSayDie

    matcher :segv do
      match do |l|
        expect(l).to raise_error(SEGV)
      end

      failure_message do
        'Expected block to SEGV'
      end

      failure_message_when_negated do
        ''
      end

      def supports_block_expectations?
        true
      end
    end
  end
end

RSpec.configure do |c|
  c.include HelixRuntime::SpecHelpers
end

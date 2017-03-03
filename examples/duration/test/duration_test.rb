require 'test_helper'

class DurationTest < Minitest::Test
  def test_blank
    assert_equal 'zomg', Duration.new.inspect
  end
end

require 'test_helper'

class TurboBlankTest < Minitest::Test
  def test_blank
    assert_predicate '', :blank?
    assert_predicate '   ', :blank?
    assert_predicate "  \n\t  \r ", :blank?
    assert_predicate '　', :blank?
    assert_predicate "\u00a0", :blank?
  end

  def test_not_blank
    refute_predicate 'a', :blank?
  end
end

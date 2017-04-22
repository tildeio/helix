require "helix_runtime"

RubyString = String

require "turbo_blank/native"

class String
  BLANK_RE = /\A[[:space:]]*\z/

  def rails_4_2_blank?
    BLANK_RE === self
  end

  def rails_5_blank?
    empty? || BLANK_RE === self
  end

  case ENV["IMPLEMENTATION"]
  when "RUST"
  when "RAILS_4_2"
    alias blank? rails_4_2_blank?
  when "RAILS_5"
    alias blank? rails_5_blank?
  when "NONE"
  else
    puts "\nPlease specify an IMPLEMENTATION: RUST, RAILS_4_2, RAILS_5 or NONE"
    exit!
  end
end

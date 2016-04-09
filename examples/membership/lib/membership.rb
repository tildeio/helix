require 'helix_runtime'
require 'membership/native'

class Array
  def naive_superset_of?(needle)
    self & needle == needle
  end

  def fast_superset_of?(needle)
    return true if needle.empty?
    return false if self.empty?

    needle_length = needle.length

    return false if needle_length > self.length

    needle_position = 0
    needle_item = needle[needle_position]

    self.each do |item|
      if item == needle_item
        needle_position += 1

        if needle_position >= needle_length
          return true
        else
          needle_item = needle[needle_position]
        end
      end
    end

    false
  end

  case ENV["IMPLEMENTATION"]
  when "RUST"
    alias superset_of? is_superset_of
  when "FAST_RUBY"
    alias superset_of? fast_superset_of?
  when "NAIVE_RUBY"
    alias superset_of? naive_superset_of?
  when "NONE"
  else
    puts "\nPlease specify an IMPLEMENTATION: RUST, FAST_RUBY, NAIVE_RUBY or NONE"
    exit!
  end
end

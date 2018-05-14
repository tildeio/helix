# require 'pry'
puts "*** USING RUBY IMPL ***"

module Journey
  module Ruby
    class Scanner
      TOKEN_MAP = {
        "/" => :SLASH,
        "*" => :STAR,
        "(" => :LPAREN,
        ")" => :RPAREN,
        "." => :DOT,
        ":" => :SYMBOL
      }

      def initialize
        @pattern = ""
        @index = 0
      end

      def scan_setup(pattern)
        @pattern = pattern
        @index = 0
      end

      def next_token
        if @index < @pattern.length
          token = @pattern[@index]
          next_token = peek_next_token

          # Increment the index in preparation for next iteration.
          @index += 1


          # binding.pry
          if determine_type(next_token) === nil
            return
          elsif (determine_type(token) == :LITERAL) && (determine_type(next_token) == :LITERAL)
            # need to handle building a word here.
            build_word(token, next_token)
            # binding.pry
          else
            [determine_type(token), token]
          end

          # if token == "/"
          #   @index += 1
          #   [:SLASH, token]
          # else
          #   binding.pry
          #   nil
          # end
        end
      end

      private

      def determine_type(token)
        if token.blank?
          nil
        elsif TOKEN_MAP.key?(token)
          TOKEN_MAP[token]
        else
          :LITERAL
        end
      end

      def peek_next_token
        # Check what the next token is.
        next_index = @index + 1
        @pattern[next_index]
      end

      def build_word(first_char, second_char)
        # binding.pry
      end
    end
  end
end

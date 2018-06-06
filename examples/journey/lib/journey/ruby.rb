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

          if determine_type(next_token) === nil
            return
          elsif (determine_type(token) == :LITERAL) && (determine_type(next_token) == :LITERAL)
            # Begin building the word by starting with the current index.
            word_end_index = @index

            # Continue incrementing the offset until we reach the end of word.
            while determine_type(peek_at_index(word_end_index + 1)) == :LITERAL
              word_end_index += 1
            end

            # Build the word literal and set the index to the offset of word literal.
            word = word_from_offset(@index - 1, word_end_index)
            @index = word_end_index

            [:LITERAL, word]
          else
            [determine_type(token), token]
          end
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

      # Check a token at a specific index.
      def peek_at_index(index)
        @pattern[index]
      end

      # Check what the next token is.
      def peek_next_token
        next_index = @index + 1
        @pattern[next_index]
      end

      def word_from_offset(beginning_index, ending_index)
        @pattern[beginning_index, ending_index]
      end
    end
  end
end

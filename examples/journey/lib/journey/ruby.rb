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
            # need to handle building a word here.
            # @index is where the word starts
            # word_end_index is where the word ends

            # if next_token is a char, don't consume it but add it to the word.
            word_end_index = @index

            while determine_type(peek_at(word_end_index + 1)) == :LITERAL
              word_end_index += 1
            end

            word = build_word(@index - 1, word_end_index)
            @index = word_end_index
            [:LITERAL, word]
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

      def peek_at(index)
        @pattern[index]
      end

      def peek_next_token
        # Check what the next token is.
        next_index = @index + 1
        @pattern[next_index]
      end

      def build_word(beginning_index, ending_index)
        @pattern[beginning_index, ending_index]
      end
    end
  end
end

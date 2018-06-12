require 'pry'
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
          elsif (determine_type(token) == :STAR) && (determine_type(next_token) == :LITERAL)
            # binding.pry
            [:STAR, consumed_word]
          elsif (determine_type(token) == :SYMBOL) && (determine_type(next_token) == :LITERAL)
            # binding.pry
            [:SYMBOL, consumed_word]
          elsif (determine_type(token) == :LITERAL) && (determine_type(next_token) == :LITERAL)
            [:LITERAL, consumed_word]
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
        binding.pry
        @pattern[beginning_index, ending_index]
      end

      def consumed_word
        puts " "
        puts " "
        puts "@pattern is: #{@pattern}"
        puts "*** ENTERING consumed_word method ***"

        # Begin building the word by starting with the current index.
        word_end_index = @index
        puts "**@index is: #{@index}, and item at @index is #{peek_at_index(@index)}"
        puts "**word_end_index is: #{word_end_index}"
        puts "**item at next index #{word_end_index + 1} is: #{peek_at_index(word_end_index + 1)}"

        # FIXME: WHAT ABOUT WHEN THE END OF THE WORD IS ALSO END OF STRING?

        # Continue incrementing the offset until we reach the end of word.
        puts "**the item at #{word_end_index + 1} is of the type: #{determine_type(peek_at_index(word_end_index + 1))}"
        while determine_type(peek_at_index(word_end_index + 1)) == :LITERAL
          # binding.pry
          puts "~~the item at #{word_end_index + 1} is #{peek_at_index(word_end_index + 1)} and is of the type: #{determine_type(peek_at_index(word_end_index + 1))}~~"

          word_end_index += 1
          puts "**I JUST INCREMENTED word_end_index, it is now: #{word_end_index}"
        end

        # binding.pry

        # Build the word literal and set the index to the offset of word literal.
        word = word_from_offset(@index - 1, word_end_index)
        puts "** WORD is: #{word}, between #{@index - 1} and #{word_end_index}"

        @index = word_end_index
        puts "** I SET @index to be word_end_index: #{word_end_index}"

        # binding.pry

        word
      end
    end
  end
end

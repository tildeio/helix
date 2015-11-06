require "strscan"

module TurboRuby
  def self.compile(input_folder:, output_folder:)
  end

  module Syntax
    class Class < Struct.new(:name)
      attr_reader :methods

      def initialize(*)
        super
        @methods = []
      end
    end

    class Method < Struct.new(:name, :return_type)
      attr_reader :body, :arguments

      def initialize(*)
        super
        @body = []
        @arguments = []
      end

      def add_argument(arg)
        @arguments << arg
      end
    end

    Argument = Struct.new(:name, :type)
  end

  class Parser
    attr_reader :ast

    def initialize
      @ast = []
    end

    def parse(source)
      @scanner = StringScanner.new(source)
      @stack = []
      @state = :program
      @indentation_pattern = nil

      until @scanner.eos?
        #puts "Processing #{@state} for #{peek_token.inspect}"
        send @state
      end

      finalize
      @ast
    ensure
      @scanner = nil
      @stack = nil
      @state = nil
      @indentation_pattern = nil
    end

    private
    def program
      consume_whitespace

      unless @scanner.eos?
        case consume_token!(/class/)
        when 'class'
          consume_whitespace!
          @state = :class_def
        end
      end
    end

    def class_def
      class_name = consume_token!(/[A-Z]\S+/)
      consume_whitespace!

      @stack.push Syntax::Class.new(class_name)
      @state = :class_body
    end

    def class_body
      case consume_token!(/def|end/)
      when 'def'
        consume_whitespace!
        @state = :method_def
      when 'end'
        @ast << @stack.pop
        @state = :program
      end
    end

    def method_def
      method_name = consume_token!

      @stack.push Syntax::Method.new(method_name)

      consume_whitespace(newlines: false)

      if consume_token(/\(/)
        consume_whitespace
        @state = :method_arguments_list_begin
      else
        @state = :method_arguments_list_end
      end
    end

    def method_arguments_list_begin
      if consume_token(/\)/)
        consume_whitespace(newlines: false)
        @state = :method_arguments_list_end
      else
        @state = :method_argument_begin
      end
    end

    def method_argument_begin
      argument_name = consume_token!

      @stack.push Syntax::Argument.new(argument_name)

      consume_whitespace(newlines: false)

      if consume_token(/->/)
        consume_whitespace
        @state = :method_argument_type
      else
        @state = :method_argument_end
      end
    end

    def method_argument_type
      argument_type = consume_token!

      @stack.last.type = argument_type

      consume_whitespace
      @state = :method_argument_end
    end

    def method_argument_end
      argument = @stack.pop
      method = @stack.last

      method.add_argument(argument)

      case consume_token!(/,|\)/)
      when ','
        consume_whitespace
        @state = :method_argument_begin
      when ')'
        consume_whitespace(newlines: false)
        @state = :method_arguments_list_end
      end
    end

    def method_arguments_list_end
      if consume_token(/->/)
        consume_whitespace
        @state = :method_return_type
      else
        @state = :method_signature_end
      end
    end

    def method_return_type
      return_type = consume_token!

      @stack.last.return_type = return_type

      consume_whitespace(newlines: false)
      @state = :method_signature_end
    end

    def method_signature_end
      consume_token!(/\n/)
      @state = :method_body_begin
    end

    def method_body_begin
      @indentation_pattern = /^#{ Regexp.escape(peek_token!(/\s+/)) }/
      @state = :method_body
    end

    def method_body
      if consume_token(/\s*?\n/)
        @state = :method_body
      elsif peek_token(@indentation_pattern)
        @stack.last.body << consume_token(/.*?\n/)
        @state = :method_body
      else
        consume_whitespace(newlines: false)
        @state = :method_body_end
      end
    end

    def method_body_end
      consume_token!(/end/)

      method = @stack.pop
      klass = @stack.last

      klass.methods << method

      consume_whitespace!
      @state = :class_body
    end

    def finalize
      raise "Unexpected EOF" unless @state == :program
    end

    def peek_token(pattern = /\S+/)
      @scanner.check(pattern)
    end

    def peek_token!(*args)
      peek_token(*args).tap do |token|
        raise "Expected to find #{pattern.inspect}" if token.nil?
      end
    end

    def consume_token(pattern = /\S+/)
      @scanner.scan(pattern)
    end

    def consume_token!(*args)
      consume_token(*args).tap do |token|
        raise "Expected to find #{args.inspect} at #{peek_token.inspect} (state: #{@state})" if token.nil?
      end
    end

    def consume_whitespace(newlines: true)
      consume_token( newlines ? /\s*/ : /[\s&&[^\n]]*/ )
    end

    def consume_whitespace!(*args)
      consume_whitespace(*args).tap do |token|
        raise "Expected to find at least one whitespace character" if token.nil?
      end
    end
  end
end

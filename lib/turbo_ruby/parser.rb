require "strscan"

module TurboRuby
  module Syntax
    Tokens = {
      K_CLASS:            /class/,
      K_DEF:              /def/,
      K_END:              /end/,

      T_CONSTANT:         /[A-Z][\w\u0080-\u{10ffff}]*/u,
      T_IDENTIFIER:       /[\w\u0080-\u{10ffff}]+/u,
      T_METHOD_NAME:      Regexp.union(
                            /[\w\u0080-\u{10ffff}]+[\?!=]?/,
                            /!/, /~/, /\+@/, /\*\*/, /-@/,
                            /\*/, /\//, /%/, /\+/, /-/,
                            /<</, />>/, /&/, /\|/, /\^/,
                            /<=>/, /===/, /==/, /</, />/,
                            /<=/, />=/, /!=/, /=~/, /!~/
                          ),

      T_COMMA:            /,/,
      T_LPAREN:           /\(/,
      T_RPAREN:           /\)/,
      T_ARROW:            /->/, # T_LAMBDA

      T_BLANKLINE:        /[\s&&[^\n]]*\n/,
      T_NEWLINE:          /\n/,
      T_SPACE:            /[\s&&[^\n]]+/,
      T_MULTILINE_SPACE:  /\s+/,

      T_RUST_BEGIN:       /<<-RUST\n/,
      T_RUST_END:         /\s*RUST\n/,

      T_OPAQUE:           /\S+/,
      T_OPAQUE_LINE:      /[^\n]*\n/
    }

    class AST
      attr_reader :nodes

      def initialize(nodes = [])
        @nodes = nodes
      end

      def to_a
        @nodes
      end

      def ==(other)
        other.class == AST &&
          self.nodes == other.nodes
      end

      module Node
        def ==(other)
          self.class == other.class
        end
      end
    end

    class Class < Struct.new(:name, :methods)
      include AST::Node

      def initialize(name, methods = [])
        super
      end

      def ==(other)
        super &&
          self.name == other.name &&
          self.methods == other.methods
      end
    end

    class Method < Struct.new(:name, :arguments, :return_type, :body)
      include AST::Node

      def initialize(name, arguments = [], return_type = nil, body = [])
        super
      end

      def ==(other)
        super &&
          self.name == other.name &&
          self.arguments == other.arguments &&
          self.return_type == other.return_type
          self.body == other.body
      end
    end

    class Argument < Struct.new(:name, :type)
      include AST::Node

      def initialize(name, type = nil)
        super
      end

      def ==(other)
        super &&
          self.name == other.name &&
          self.type == other.type
      end
    end
  end

  class ParseError < RuntimeError; end

  class Parser

    attr_reader :ast

    def initialize
      @ast = Syntax::AST.new
    end

    def parse(source)
      @scanner = StringScanner.new(source)
      @stack = []
      @state = :program

      until @scanner.eos?
        # puts "Current state: #{@state}; Next token: #{next_token.inspect}"
        send @state
      end

      finalize

      @ast
    ensure
      @scanner = nil
      @stack = nil
      @state = nil
    end

    private
    def program
      consume_whitespace

      unless @scanner.eos?
        consume_token!(:K_CLASS)
        consume_whitespace!(multiline: false)
        @state = :class_def
      end
    end

    def class_def
      @stack.push Syntax::Class.new(consume_token!(:T_CONSTANT))
      consume_whitespace!
      @state = :class_body
    end

    def class_body
      case consume_token!(:K_DEF, :K_END)
      when 'def'
        consume_whitespace!(multiline: false)
        @state = :method_def
      when 'end'
        @ast.nodes << @stack.pop
        @state = :program
      end
    end

    def method_def
      @stack.push Syntax::Method.new(consume_token!(:T_METHOD_NAME))

      if consume_token(:T_LPAREN)
        consume_whitespace
        @state = :method_arguments_list_begin
      else
        consume_whitespace(multiline: false)
        @state = :method_arguments_list_end
      end
    end

    def method_arguments_list_begin
      if consume_token(:T_RPAREN)
        consume_whitespace(multiline: false)
        @state = :method_arguments_list_end
      else
        @state = :method_argument_begin
      end
    end

    def method_argument_begin
      @stack.push Syntax::Argument.new(consume_token!(:T_IDENTIFIER))

      consume_whitespace(multiline: false)

      if consume_token(:T_ARROW)
        consume_whitespace(multiline: false)
        @state = :method_argument_type
      else
        @state = :method_argument_end
      end
    end

    def method_argument_type
      @stack.last.type = consume_token!(:T_OPAQUE)
      consume_whitespace
      @state = :method_argument_end
    end

    def method_argument_end
      argument = @stack.pop
      method = @stack.last
      method.arguments << argument

      case consume_token!(:T_COMMA, :T_RPAREN)
      when ','
        consume_whitespace
        @state = :method_argument_begin
      when ')'
        consume_whitespace(multiline: false)
        @state = :method_arguments_list_end
      end
    end

    def method_arguments_list_end
      if consume_token(:T_ARROW)
        consume_whitespace(multiline: false)
        @state = :method_return_type
      else
        @state = :method_signature_end
      end
    end

    def method_return_type
      @stack.last.return_type = consume_token!(:T_OPAQUE)
      consume_whitespace(multiline: false)
      @state = :method_signature_end
    end

    def method_signature_end
      consume_token!(:T_NEWLINE)
      consume_whitespace
      @state = :method_body_begin
    end

    def method_body_begin
      if consume_token(:T_RUST_BEGIN)
        @state = :rust_method_body
      else
        @state = :method_body_end
      end
    end

    def rust_method_body
      if consume_token(:T_RUST_END)
        consume_whitespace
        @state = :method_body_end
      else
        @stack.last.body << consume_token!(:T_OPAQUE_LINE)
        @state = :rust_method_body
      end
    end

    def method_body_end
      consume_token!(:K_END)

      method = @stack.pop
      klass = @stack.last
      klass.methods << method

      consume_whitespace!
      @state = :class_body
    end

    def finalize
      raise ParseError, "Unexpected EOF" unless @state == :program
    end

    def peek_token(*labels)
      @scanner.check(regexp_from_labels(labels))
    end

    def consume_token(*labels)
      @scanner.scan(regexp_from_labels(labels))
    end

    def consume_token!(*labels)
      consume_token(*labels).tap do |token|
        raise ParseError, "Expected to find #{inspect_labels(labels)} but found #{next_token.inspect} instead (state=#{@state})" if token.nil?
      end
    end

    def consume_whitespace(multiline: true)
      consume_token( multiline ? :T_MULTILINE_SPACE : :T_SPACE )
    end

    def consume_whitespace!(multiline: true)
      consume_token!( multiline ? :T_MULTILINE_SPACE : :T_SPACE )
    end

    def next_token
      peek_token(:T_OPAQUE_LINE)
    end

    def regexp_from_labels(labels)
      labels = [:T_OPAQUE] if labels.empty?
      tokens = labels.map(&method(:token_from_label))
      Regexp.union(tokens)
    end

    def inspect_labels(labels)
      if labels.size <= 1
        labels.first || :T_OPAQUE
      else
        labels = labels.dup
        last = labels.pop
        "#{labels.join(', ')} or #{last}"
      end
    end

    def token_from_label(name)
      Syntax::Tokens[name]
    end
  end
end

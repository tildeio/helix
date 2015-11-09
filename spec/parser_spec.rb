require 'spec_helper'

module TurboRuby
  describe Parser do
    let(:fast_blank_trb) { File.read(File.expand_path('../fixtures/fast_blank/src/fast_blank.trb', __FILE__)) }

    it 'parses fast_blank.trb' do
      subject.parse(fast_blank_trb)

      expect(subject.ast).to eq(ast([
        klass('String', [
          method('blank?', [], 'bool', [
            "      self.chars().all(|c| c.is_whitespace())\n"
          ])
        ])
      ]))
    end

    it 'parses a blank files' do
      subject.parse('')
      subject.parse('                ')
      subject.parse(" \n \n \t \r\n  ")

      expect(subject.ast).to eq(ast([]))
    end

    it "does not parse incomplete source" do
      incomplete_sources = [
        'class',
        'class ',
        'class MyClass',
        'class MyClass ',
        "class MyClass\n  def",
        "class MyClass\n  def ",
        "class MyClass\n  def zomg",
        "class MyClass\n  def zomg ",
        "class MyClass\n  def zomg\n  end",
        "class MyClass\n  def zomg\n  end "
      ]

      incomplete_sources.each do |source|
        expect { subject.parse(source) }.to raise_error(ParseError)
      end

      expect(subject.ast).to eq(ast([]))
    end

    it "parses bare methods" do
      subject.parse <<-RUBY
        class MyClass
          def zomg
          end

          def zomg()
          end

          def     zomg(              )#{'     '}
          end

          def _zomg
          end

          def zomg?
          end

          def zomg!
          end

          def zomg=
          end

          def 💩
          end

          def +@
          end

          def <=>
          end
        end
      RUBY

      expect(subject.ast).to eq(ast([
        klass('MyClass', [
          method('zomg', [], nil, []),
          method('zomg', [], nil, []),
          method('zomg', [], nil, []),
          method('_zomg', [], nil, []),
          method('zomg?', [], nil, []),
          method('zomg!', [], nil, []),
          method('zomg=', [], nil, []),
          method('💩', [], nil, []),
          method('@+', [], nil, []),
          method('<=>', [], nil, [])
        ])
      ]))
    end

    it "does not parse invalid methods" do
      invalid_names = [
        '',
        '     ',
        '!zomg',
        '?zomg',
        '=zomg',
        '?',
        '=',
        '@+', # should be +@
        '>=<'
      ]

      invalid_names.each do |name|
        expect { subject.parse("class MyClass\n  def #{name}\n  end\nend") }.to raise_error(ParseError)
      end

      expect(subject.ast).to eq(ast([]))
    end

    def ast(*args)
      Syntax::AST.new(*args)
    end

    def klass(*args)
      Syntax::Class.new(*args)
    end

    def method(*args)
      Syntax::Method.new(*args)
    end

    def argument(*args)
      Syntax::Argument.new(*args)
    end
  end
end
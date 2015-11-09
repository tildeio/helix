require 'spec_helper'

module TurboRuby
  describe Compiler do
    context "fast_blank" do
      let(:input) { FileSystem::InMemoryTree.from_root(File.expand_path('../fixtures/fast_blank', __FILE__)) }
      let(:output) { FileSystem::InMemoryTree.new }

      def compile!
        Compiler.new(input, output)
      end

      it 'copies over the gemspec' do
        compile!
        expect(output.file?('fast_blank.gemspec')).to be_truthy
        expect(output.read('fast_blank.gemspec')).to eq(input.read('fast_blank.gemspec'))
      end

      it 'raises when the gemspec is missing' do
        input.remove('fast_blank.gemspec')
        expect { compile! }.to raise_error(/Expected to find a single gemspec file but none was present/)
      end

      it 'raises when there are too many gemspecs' do
        input.write('another_fast_blank.gemspec', 'zomg')
        expect { compile! }.to raise_error(/Expected to find a single gemspec file but found 2 instead/)
      end

      it 'raises when the gemspec is invalid' do
        input.write('fast_blank.gemspec', 'zomg')
        expect { compile! }.to raise_error(/fast_blank.gemspec is not a valid gemspec/)
      end

      it 'generates a Cargo.toml' do
        compile!
        expect(output.file?('Cargo.toml')).to be_truthy
        expect(output.read('Cargo.toml')).to eq(<<TOML)
[package]
name = "fast_blank"
version = "1.0.0"
authors = ["Sam Saffron <sam.saffron@gmail.com>"]

[lib]
name = "fast_blank"
doctest = false
crate-type = ["staticlib"]

[dependencies]

libc = "=0.2.1"
TOML
      end

      it 'generates a extconf.rb' do
        compile!
        expect(output.file?('extconf.rb')).to be_truthy
        expect(output.read('extconf.rb')).to eq(<<EXTCONF)
require "mkmf"

LIB_DIRS = [
  File.expand_path("../target/release", __FILE__)
]

dir_config("fast_blank", nil, LIB_DIRS)

$LOCAL_LIBS << "-lfast_blank"

create_makefile "fast_blank"
EXTCONF
      end
    end
  end
end
require 'spec_helper'
require 'active_support/core_ext/string/strip'

module TurboRuby
  describe Compiler do
    context "fast_blank" do
      let(:input) { FileSystem::InMemoryTree.from_root(File.expand_path('../fixtures/fast_blank', __FILE__)) }
      let(:output) { FileSystem::InMemoryTree.new }

      def compile!
        Compiler.new(input, output)
      end

      context "error cases" do
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
      end

      context "successful compilation" do
        before do
          compile!
        end

        it 'copies over the gemspec' do
          expect(output.file?('fast_blank.gemspec')).to be_truthy
          expect(output.read('fast_blank.gemspec')).to eq(input.read('fast_blank.gemspec'))
        end

        it 'generates a Cargo.toml' do
          expect(output.read('Cargo.toml')).to eq(<<-TOML.strip_heredoc)
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
            ffi_buf = "1.0.0"
          TOML
        end

        it 'generates a extconf.rb' do
          expect(output.read('extconf.rb')).to eq(<<-EXTCONF.strip_heredoc)
            require "mkmf"

            LIB_DIRS = [
              File.expand_path("../target/release", __FILE__)
            ]

            dir_config("fast_blank", nil, LIB_DIRS)

            $LOCAL_LIBS << "-lfast_blank"

            create_makefile "fast_blank"
          EXTCONF
        end

        it 'generates a fast_blank.c file' do
          expect(output.read('fast_blank.c')).to eq(<<-FAST_BLANK.strip_heredoc)
            #include <ruby.h>
            #include <ruby/encoding.h>

            typedef struct {
              void* ptr;
              size_t len;
            } trb_buf_t;

            static inline trb_buf_t
            STR2BUF(VALUE str) {
              return (trb_buf_t) {
                .ptr = RSTRING_PTR(str),
                .len = RSTRING_LEN(str),
              };
            }

            int trb_string_is_blank(trb_buf_t self);

            static VALUE
            string_is_blank(VALUE self) {
              return (trb_string_is_blank(STR2BUF(self)) ? Qtrue : Qfalse);
            }

            void Init_fast_blank( void ) {
              rb_define_method(rb_cString, "blank?", string_is_blank, 0);
            }
          FAST_BLANK
        end

        it 'generates a src/lib.rs file' do
          expect(output.read('src/lib.rs')).to eq(<<-LIBRS.strip_heredoc)
            extern crate libc;
            extern crate ffi_buf;

            use ffi_buf::Buf;

            trait RubyString {
                fn trb_is_blank(&self) -> bool;
            }

            impl<'a> RubyString for &'a str {
                fn trb_is_blank(&self) -> bool {
                    self.chars().all(|c| c.is_whitespace())
                }
            }

            #[no_mangle]
            pub extern "C" fn trb_string_is_blank(rb_self: Buf) -> bool {
                rb_self.as_slice().trb_is_blank()
            }
          LIBRS
        end
      end
    end
  end
end

require 'spec_helper'
require 'active_support/core_ext/string/strip'

module TurboRuby
  describe Compiler do
    let(:ast) do
      trb = File.read(File.expand_path('../fixtures/fast_blank/src/fast_blank.trb', __FILE__))
      Parser.new.parse(trb)
    end

    describe Compiler::Method do
      context "fast_blank" do
        let(:ast_node) do
          ast.nodes.first.methods.first
        end

        subject do
          described_class.new(ast_node, klass_name: "String", project_prefix: 'fast_blank')
        end

        it "emits the Init_ definition" do
          expect(subject.init_method).to eq('rb_define_method(fast_blank_String, "blank?", string_blank_p, 0);')
        end

        it "emits the static C function" do
          expect(subject.static_function).to eq(<<-C.strip_heredoc.chomp)
            static VALUE
            string_blank_p(VALUE self) {
              bool ret;
              CHECK_FFI(fast_blank_string_blank_p(STR2BUF(self), &ret));
              return BOOL2RB(ret);
            }
          C
        end

        it "emits the Rust prototype" do
          expect(subject.c_rust_prototype).to eq('boolean fast_blank_string_blank_p(trb_buf_t self, boolean*);')
        end

        it "emits the Rust no_mangle'd function" do
          expect(subject.rust_extern_fn).to eq(<<-RUST.strip_heredoc.chomp)
            #[no_mangle]
            pub extern "C" fn fast_blank_string_blank_p(rb_self: Buf, out: *mut bool) -> Option<ErrorDesc> {
              unsafe { *ret = rb_self.as_slice().fast_blank_blank_p() };
              None
            }
          RUST
        end

        it "emits the Rust trait method implementation" do
          expect(subject.rust_trait_impl).to eq(<<-RUST.strip_heredoc.chomp)
            fn fast_blank_blank_p(&self) -> bool {
                self.chars().all(|c| c.is_whitespace())
            }
          RUST
        end

        it "emits the Rust trait method definition" do
          expect(subject.rust_trait_def).to eq('fn fast_blank_blank_p(&self) -> bool;')
        end
      end
    end

    describe Compiler::Class do
      context "fast_blank" do
        let(:ast_node) { ast.nodes[0] }
        subject { described_class.new(ast_node, project_prefix: 'fast_blank') }

        it "emits the correct constant lookup code" do
          expect(subject.c_const_lookup).to eq(<<-C.strip_heredoc.chomp)
            ID fast_blank_String_ID = rb_intern("String");
            VALUE fast_blank_String;

            if (rb_const_defined(rb_cObject, fast_blank_String_ID)) {
              fast_blank_String = rb_const_get(rb_cObject, fast_blank_String_ID);
            } else {
              rb_raise(rb_eNotImpError, "Defining a new class is not currently supported");
            }
          C
        end

        it "emits the right trait wrapper" do
          expect(subject.rust_trait_definition).to eq(<<-RUST.strip_heredoc.chomp)
            trait RubyString {
                fn fast_blank_blank_p(&self) -> bool;
            }
          RUST
        end

        it "emits the right trait impl" do
          expect(subject.rust_trait_impl).to eq(<<-RUST.strip_heredoc.chomp)
            impl RubyString for str {
                fn fast_blank_blank_p(&self) -> bool {
                    self.chars().all(|c| c.is_whitespace())
                }
            }
          RUST
        end
      end
    end
  end
end

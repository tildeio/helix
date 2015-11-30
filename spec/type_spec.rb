require 'spec_helper'
require 'active_support/core_ext/string/strip'

module TurboRuby
  describe Compiler::Method do
    context "fast_blank" do
      let(:ast_node) do
        Syntax::Method.new("blank?", [], "bool", ["    self.chars().all(|c| c.is_whitespace())\n"])
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
end

#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

#[derive(Debug)]
struct NotClonable();

ruby! {
    class ClassContainingNonClonableFields {
        struct {
            inner: NotClonable,
        }

        def initialize(helix) {
            ClassContainingNonClonableFields { helix, inner: NotClonable {} }
        }

        def to_s(&self) -> &'static str {
            "ClassContainingNonClonableFields"
        }
    }
}

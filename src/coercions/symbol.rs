use sys::{VALUE, RB_TYPE_P, T_SYMBOL, rb_sym2id, rb_id2sym};
use super::{FromRuby, CheckedValue, CheckResult, ToRuby, ToRubyResult};
use super::super::{Symbol};

impl FromRuby for Symbol {
    type Checked = CheckedValue<Symbol>;

    fn from_ruby(value: VALUE) -> CheckResult<CheckedValue<Symbol>> {
        if unsafe { RB_TYPE_P(value, T_SYMBOL) } {
            unsafe { Ok(CheckedValue::new(value)) }
        } else {
            type_error!(value, "a symbol")
        }
    }

    fn from_checked(checked: CheckedValue<Symbol>) -> Symbol {
        Symbol::from_id(unsafe { rb_sym2id(checked.to_value()) })
    }
}

impl ToRuby for Symbol {
    fn to_ruby(self) -> ToRubyResult {
        Ok(unsafe { rb_id2sym(self.to_id()) })
    }
}

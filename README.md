Read the [*Introducing Helix*](http://blog.skylight.io/introducing-helix/) blog post for a quick introduction to the project!

* * *

[![Travis Build Status](https://travis-ci.org/tildeio/helix.svg?branch=master)](https://travis-ci.org/tildeio/helix)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/tildeio/helix?branch=master&svg=true)](https://ci.appveyor.com/project/wagenet/helix)

WARNING: This repository is still in active development. **The vast majority of important Ruby
APIs are not yet supported**, because we are still in the process of formulating the rules for
binding Ruby APIs (so that we can make things ergonomic and provide **safety guarantees**).

What follows is an aspirational README :wink:

# Helix

Helix allows you to write Ruby classes in Rust without having to write the glue code yourself.

```rust
ruby! {
    class Console {
        def log(self, string: &str) {
            println!("LOG: {}", string);
        }
    }
}
```

```shell
$ irb
>> require "console/native"
>> Console.new.log("I'm in your Rust")
LOG: I'm in your Rust
```

> STATUS: The main thing missing from the current implementation is coercing Rust return types in Ruby. Today, you would need to add `Qnil` to the bottom of `def log`, which we hope to eliminate soon.

## Getting Started with Helix Examples
If you'd like to experiment with Helix, you can start with some of the examples in this repository.

Clone and download the Helix repository:
```shell
$ git clone https://github.com/tildeio/helix
```

Navigate to the console example folder and bundle your `Gemfile`:
```shell
$ cd examples/console
$ bundle install
```

Run `rake`, followed by `rake irb`:
```shell
$ rake
$ rake irb
```

Try running some of the methods defined in `examples/console/src/lib.rs`:
```shell
> c = Console.new
Console { helix: VALUE(0x7fdacc19a6a0) }
=>
> c.hello
hello
=> nil
> c.loglog('hello', 'world')
hello world
=> nil
```

## Coercions

When you define a method in Helix using `def`, you can specify any Rust type in its type signature.

Under the hood, Helix will automatically coerce the Ruby type to the specified Rust type, doing appropriate type checks before passing the values into Rust.

```rust
ruby! {
    class Console {
        def log(string: &str) {
            println!("LOG: {}", string);
        }
    }
}
```

```shell
$ irb
>> require "console/native"
>> Console.new.log({})
TypeError: no implicit coercion of Hash into Rust &str
	from (irb):2:in `log'
	from (irb):2
	from /Users/ykatz/.rvm/rubies/ruby-2.3.0/bin/irb:11:in `<main>'
```

> STATUS: This protocol already works now and is implemented for `String` and `&[u8]`

### The Helix Coercion Protocol

Under the hood, Helix does not hardcode all possible coercions from Ruby into Rust. Instead, it defines a two-part protocol that any crate can implement to define coercions from Ruby values into their types.

```rust
pub trait UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<T>;
}

pub trait ToRust<U, T: CheckedValue<U>> {
    fn to_rust(self) -> T;
}
```

Implementations of these traits use these concrete types:

```rust
pub type CheckResult<T> = Result<CheckedValue<T>, CString /* error */>;

pub struct CheckedValue<T> {
    pub inner: VALUE;
    // other private fields
}

impl<T> CheckedValue<T> {
    // instantiating a CheckedValue<T> is an assertion that the follow-up
    // call to `to_rust` is safe.
    pub unsafe fn new(inner: VALUE) -> CheckedValue<T>;
}
```

For reference, here is the implementation of the coercion from a Ruby `String` to Rust `String`.

```rust
impl UncheckedValue<String> for VALUE {
    fn to_checked(self) -> CheckResult<String> {
        // check whether the VALUE is actually a String
        if unsafe { sys::RB_TYPE_P(self, sys::T_STRING) } {
            // assert that we can guarantee that to_rust() can return a Rust String safely
            Ok(unsafe { CheckedValue::<String>::new(self) })
        } else {
            Err(format!("No implicit conversion of {} into Rust String", "?"))
        }
    }
}

impl ToRust<String> for CheckedValue<String> {
    fn to_rust(self) -> String {
        // we're sure that these calls are safe, because we already went through the type
        // checking protocol in VALUE.to_checked().
        let size = unsafe { sys::RSTRING_LEN(self.inner) };
        let ptr = unsafe { sys::RSTRING_PTR(self.inner) };
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
        unsafe { std::str::from_utf8_unchecked(slice) }.to_string()
    }
}
```

This protocol allows us to fully type check a method's arguments before starting any of the coercions. It happens automatically based on the type signature you use in your Rust method `def`.

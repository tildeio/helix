[![Travis Build Status](https://travis-ci.org/tildeio/helix.svg?branch=master)](https://travis-ci.org/tildeio/helix)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/tildeio/helix?branch=master&svg=true)](https://ci.appveyor.com/project/wagenet/helix)

# Helix

Helix allows you to write Ruby classes in Rust without having to write the glue code yourself.

```rust
ruby! {
    class Console {
        def log(string: String) {
            println!("LOG: {}", string);
        }
    }
}
```

```shell
$ rake build
$ bundle exec irb
>> require "console"
>> Console.log("I'm in your Rust")
LOG: I'm in your Rust
 => nil
```

## Why Helix?

Read the [*Introducing Helix*](http://blog.skylight.io/introducing-helix/) blog post for a quick introduction to the project!

## Getting Started

https://usehelix.com/getting_started

## Demos

https://usehelix.com/demos

## Roadmap

https://usehelix.com/roadmap


## Contributing

If you'd like to experiment with Helix, you can start with some of the examples in this repository.

Clone and download the Helix repository:
```shell
$ git clone https://github.com/tildeio/helix
$ cd helix
```

Navigate to the console example folder and bundle your `Gemfile`:
```shell
$ cd examples/console
$ bundle install
```

Run `rake irb` to build and start irb:
```shell
$ bundle exec rake irb
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
        def log(&self, string: &str) {
            println!("LOG: {}", string);
        }
    }
}
```

```shell
$ irb
>> require "console"
>> Console.new.log({})
TypeError: No implicit coercion of {} into String
	from (irb):2:in `log'
	from (irb):2
	from /Users/ykatz/.rvm/rubies/ruby-2.3.0/bin/irb:11:in `<main>'
```

### The Helix Coercion Protocol

Under the hood, Helix does not hardcode all possible coercions from Ruby into Rust. Instead, it defines a two-part protocol that any crate can implement to define coercions from Ruby values into their types.

```rust
pub trait UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<T>;
}

pub trait ToRust<T> {
    fn to_rust(self) -> T;
}
```

Implementations of these traits use these concrete types:

```rust
pub type CheckResult<T> = Result<CheckedValue<T>, String /* error */>;

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
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into String", val.to_rust()))
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

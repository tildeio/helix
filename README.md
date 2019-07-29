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

## Compatibility

Helix has been tested with the following, though other combinations may also work.

* cargo 0.18.0 (fe7b0cdcf 2017-04-24)
* rustc 1.17.0 (56124baa9 2017-04-24)
* ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-darwin16]
* Bundler version 1.14.6

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

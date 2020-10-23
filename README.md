[![Travis Build Status](https://travis-ci.org/tildeio/helix.svg?branch=master)](https://travis-ci.org/tildeio/helix)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/tildeio/helix?branch=master&svg=true)](https://ci.appveyor.com/project/wagenet/helix)

# ~~Helix~~

> :warning: **Deprecated** :warning:
>
> Sadly, we have made the decision to deprecate this project. While we had
> hoped to bootstrap the project to a point where it could flourish as a
> community project, unfortunately we ran into a number of roadblocks along the
> way, along with the need for significant architectural overhaul. While these
> issues are solvable on a technical level, doing so correctly requires more
> resources than we have been able to provide and progress has stalled.
>
> One of our goals was also to integrate our own Skylight agent with Helix,
> aligning the company's priorities with the project. While the Skylight agent
> is still written in Rust with a thin layer of C bindings (which is the part
> Helix would replace), we were not able to get the project to the point where
> we felt comfortable running it on our customer's servers. We did not identify
> any specific blockers that would prevent us from doing this, but ultimate, we
> did not have the necessary time and resources to realize this.
>
> Since we are a small team, it is unlikely that we will be able to provide the
> necessary investment in the foreseeable future to achieve our ambitions for
> the project. At this point, we believe it is in everyone's best interest to
> formally deprecate the project, accurately reflecting its effective state.
> Meanwhile, others in the Ruby and Rust communities have continued to explore
> in the adjacent research areas. Some of them have made great progress and
> brought new ideas and innovations to the table. We look forward to seeing
> these new ideas come to fruition and fill the void we are leaving.

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

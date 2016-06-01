# Running the examples

Tested with:


cargo 0.11.0-nightly (3ff108a 2016-05-24)

rustc 1.11.0-nightly (a967611d8 2016-05-30)

ruby 2.3.1p112 (2016-04-26 revision 54768)

Bundler version 1.12.5

### Main
1. In the main project `cargo build --release`

### Ruby gem
1. Go to helix/ruby
2. `bundle install`
  * [requires neversaydie](https://github.com/tenderlove/neversaydie)
3. `bundle exec rake`

### Example
1. go to helix/examples/project-name
2. `bundle install`
3. `bundle exec gem install ../../ruby/pkg/helix_runtime-0.5.0.gem`
4. `cargo build --release`
5. `bundle exec rake irb IMPLEMENTATION=RUST`


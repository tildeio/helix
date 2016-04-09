require "pp"
require "json"

# puts "cargo:rustc-link-lib=static=/Users/pairing/.rvm/rubies/ruby-2.2.2/lib/libruby-static.a"

puts JSON.pretty_generate(RbConfig::CONFIG)

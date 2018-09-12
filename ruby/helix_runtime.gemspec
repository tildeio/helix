# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'helix_runtime/version'

Gem::Specification.new do |spec|
  spec.name          = "helix_runtime"
  spec.version       = HelixRuntime::GEM_VERSION
  spec.authors       = ["Yehuda Katz", "Godfrey Chan"]
  spec.email         = ["wycats@gmail.com", "godfreykfc@gmail.com"]

  spec.summary       = %q{The Helix Runtime}
  spec.homepage      = "https://usehelix.com"

  spec.metadata      = {
    "bug_tracker_uri" => "https://github.com/tildeio/helix/issues",
    "changelog_uri"   => "https://github.com/tildeio/helix/blob/master/CHANGELOG.md",
    "source_code_uri" => "https://github.com/tildeio/helix",
  }

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = "bin"
  # spec.extensions    = ["ext/helix_runtime/native/extconf.rb"]
  spec.executables   = spec.files.grep(%r{^bin/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_dependency "libcruby_sys"
  spec.add_dependency "rake",   ">= 10.0"
  spec.add_dependency "thor",   [">= 0.19.4", "< 2.0"]
  spec.add_dependency "tomlrb", "~> 1.2.4"

  spec.add_development_dependency "bundler", "~> 1.10"
  spec.add_development_dependency "rspec", "~> 3.4"
  spec.add_development_dependency "rake-compiler", "~> 0.9.7"
end

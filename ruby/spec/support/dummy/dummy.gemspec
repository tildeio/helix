# coding: utf-8

Gem::Specification.new do |spec|
  spec.name          = "dummy"
  spec.version       = "1.0.0"
  spec.authors       = ["Yehuda Katz", "Godfrey Chan"]
  spec.email         = ["wycats@gmail.com", "godfreykfc@gmail.com"]

  spec.summary       = "A dummy gem that re-export the C variables and functions to Ruby-land."

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = "exe"
  spec.extensions    = ["ext/dummy/extconf.rb"]
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.add_runtime_dependency "helix_runtime", "*"
end

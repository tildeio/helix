Gem::Specification.new do |s|
  s.name = 'fast_blank'
  s.version = '1.0.0'
  s.date = '2015-08-03'
  s.summary = 'Fast String blank? implementation'
  s.description = 'Provides a native method for determining if a string is blank'

  s.authors = ['Sam Saffron']
  s.email = ['sam.saffron@gmail.com']
  s.homepage = 'https://github.com/SamSaffron/fast_blank'
  s.license = 'MIT'

  s.extensions = ['ext/fast_blank/extconf.rb']
  s.require_paths = ['lib']
  s.files = [
    'MIT-LICENSE',
    'README.md',
    'benchmark',
    'lib/.gemkeep',
    'ext/fast_blank/fast_blank.c',
    'ext/fast_blank/extconf.rb',
  ]
  s.test_files = Dir['spec/**/*_spec.rb']

  s.platform = Gem::Platform::RUBY
  s.rubygems_version = '1.3.7'

  s.add_development_dependency 'rake-compiler'
  s.add_development_dependency 'rspec'
  s.add_development_dependency 'benchmark-ips'
end
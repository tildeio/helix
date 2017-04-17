# encoding: utf-8

Gem::Specification.new do |s|
  s.name         = '<%= app_name %>'
  s.version      = '1.0.0'
  s.authors      = ['Ruby Developer']
  s.summary      = "A Helix project"
  s.files        = Dir['{lib/**/*,[A-Z]*}']

  s.platform     = Gem::Platform::RUBY
  s.require_path = 'lib'

  s.add_dependency 'helix_runtime', '~> <%= HelixRuntime::GEM_VERSION %>'
end

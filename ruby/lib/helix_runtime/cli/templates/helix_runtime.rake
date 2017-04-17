require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new("<%= app_name %>")

task :default => :build

require 'rake/tasklib'
require 'helix_runtime'

module HelixRuntime
  # FIXME: I don't like this name
  class ParentBuildTask < Rake::TaskLib

    def initialize
      define
    end

    def define
      desc "Build Helix projects in crates"
      task :build do
        project.projects.each do |p|
          puts "Building #{p.name}"
          p.build
        end
      end

      desc "Clobber Helix builds in crates"
      task :clobber do
        project.projects.each do |p|
          puts "Clobbering #{p.name}"
          p.clobber
        end
      end
    end

    private

      def project
        @project ||= ParentProject.new(Dir.pwd)
      end

  end
end
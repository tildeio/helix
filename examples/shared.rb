module HelixRuntime
  module Tests
    def self.pre_build
      -> {
        Dir.chdir("../../ruby") do
          puts "\n\nBuilding helix runtime\n\n"
          Bundler.with_clean_env do
            run_cmd "bundle install"
            run_cmd "bundle exec rake compile:native"
          end
          puts "\nRuntime built\n\n"
        end
      }
    end

    def self.run_cmd(cmd)
      puts(cmd)
      system(cmd)
    end
  end
end

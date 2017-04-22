require 'helix_runtime/cli/bootstrap'

module HelixRuntime
  module CLI
    class Base < Thor
      include Thor::Actions

      register CLI::Bootstrap, "bootstrap", "bootstrap PATH [NAME]", "Bootstrap Helix"

      desc "crate NAME", "Generate a Helix crate"
      def crate(name)
        bootstrap("crates/#{name}", name)
        append_to_file "Gemfile", "gem '#{name}', path: 'crates/#{name}'\n"
      end
    end
  end
end

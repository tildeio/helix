require 'spec_helper'
require 'tmpdir'
require 'open3'

module TurboRuby
  describe "Integration" do
    context "print_buf" do
      let(:project_name) { 'print_buf' }

      def run!(dir, *cmd, allow_stderr: false)
        stdout_str, stderr_str, status = Open3.capture3(*cmd, chdir: dir)
        expect(stderr_str).to eq('') unless allow_stderr
        expect(status).to eq(0)
        stdout_str
      end

      def build_and_run!(ruby)
        Dir.mktmpdir do |out_dir|
          in_dir = File.expand_path("../fixtures/#{project_name}", __FILE__)

          input = FileSystem::Tree.new(in_dir)
          output = FileSystem::Tree.new(out_dir)

          Compiler.new(input, output, dev: true)

          run!(out_dir, "cargo build --release", allow_stderr: true)
          run!(out_dir, RbConfig.ruby, "extconf.rb")
          run!(out_dir, "make", allow_stderr: true)

          assert(output.file?("#{project_name}.bundle"))

          run!(out_dir, *%W{ #{RbConfig.ruby} -I . -r #{project_name} -e #{ruby} })
        end
      end

      it "should work" do
        expect(build_and_run!('"zomg".print_buf')).to eq("zomg\n")
      end
    end
  end
end

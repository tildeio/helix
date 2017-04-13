task :test do
  cd "ruby" do
    sh "bundle exec rake"
  end

  examples = ENV["EXAMPLES"] || "duration calculator console membership turbo_blank"

  sh "bash ./examples/runner default #{examples}"
end

task :default => :test

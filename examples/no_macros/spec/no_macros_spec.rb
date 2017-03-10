require "spec_helper"
require "colorize"

describe "NoMacros" do
  subject { NoMacros.new }

  it "works" do
    expect(subject.hello).to eq("hello")
  end
end

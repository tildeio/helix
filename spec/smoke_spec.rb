require 'spec_helper'
require "tmpdir"

describe "Smoke test" do
  pending "compiles a native ruby dynamic library" do
    Dir.mktmpdir do |dir|
      TurboRuby.compile(File.expand_path("../fixtures/string.trb", __FILE__), out: dir)
      require File.expand_path("./string", dir)
      expect("foo".blank?).to eq(false)
      expect("".blank?).to eq(true)
    end
  end
end

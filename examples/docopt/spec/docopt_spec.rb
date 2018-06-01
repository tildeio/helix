require "spec_helper"
USAGE = <<USAGE
Naval Fate.

    Usage:
      naval_fate ship new <name>...
      naval_fate ship <name> move <x> <y> [--speed=<kn>] [--acc=<kns>]
      naval_fate ship shoot <x> <y>
      naval_fate mine (set|remove) <x> <y> [--moored | --drifting]
      naval_fate (-h | --help)
      naval_fate --version
      naval_fate (-o | --option)

    Options:
      -h --help     Show this screen.
      --version     Show version.
      --speed=<kn>  Speed in knots [default: 10].
      --acc=<kns>   Speed in knots per second.
      --moored      Moored (anchored) mine.
      --drifting    Drifting mine.
      -o --option   Test long and short option.
USAGE

describe "Docopt" do
  it "can not be constructed" do
    expect { Docopt.new }.to raise_error(NoMethodError)
  end

  it "should parse and return bools" do
    argv = "naval_fate --help".split

    options = Docopt.parse(USAGE, argv)
    expect(options["--help"]).to be true
  end

  it "should parse strings" do
    x    = "1"
    y    = "2"
    argv = "naval_fate ship shoot #{x} #{y}".split

    options = Docopt.parse(USAGE, argv)
    expect(options["ship"]).to eq(true)
    expect(options["shoot"]).to eq(true)
    expect(options["<x>"]).to eq(x)
    expect(options["<y>"]).to eq(y)
  end

  it "should set short and long options are when provided" do
    argv = "naval_fate -o".split
    options = Docopt.parse(USAGE, argv)
    expect(options["--option"]).to eq(true)
    argv = "naval_fate --option".split
    options = Docopt.parse(USAGE, argv)
    expect(options["--option"]).to eq(true)
  end

  it "should parse an array of strings" do
    names = %w(enterprise mission)
    argv  = "naval_fate ship new #{names.join(" ")}".split

    options = Docopt.parse(USAGE, argv)
    expect(options["ship"]).to eq(true)
    expect(options["<name>"]).to eq(names)
  end

  it "should use default values" do
    argv = "naval_fate ship foo move 0 0".split

    options = Docopt.parse(USAGE, argv)
    expect(options["--speed"]).to eq("10")
  end

  it "should return nil for unused options" do
    argv = "naval_fate ship foo move 0 0".split

    options = Docopt.parse(USAGE, argv)
    expect(options["--acc"]).to be_nil
  end

  it "should handle complex cases" do
    name  = "enterprise"
    x     = "1"
    y     = "2"
    speed = "10"
    argv  = "naval_fate ship #{name} move #{x} #{y} --speed=#{speed}".split

    options = Docopt.parse(USAGE, argv)
    expect(options["ship"]).to eq(true)
    expect(options["move"]).to eq(true)
    expect(options["<name>"]).to eq([name])
    expect(options["<x>"]).to eq(x)
    expect(options["<y>"]).to eq(y)
    expect(options["--speed"]).to eq(speed)
  end
end

require "spec_helper"
require "colorize"

describe "Console" do
  let(:console) { Console.new }

  it "can log a string" do
    expect { console.log("hello") }.to println("hello")
  end

  it "can log an array of strings" do
    expect { console.log_lines(["hello", "world"]) }.to println("hello\nworld")
  end

  it "can inspect itself" do
    expect { console.inspect }.to print(/Console { .+ }\n\z/)
  end

  it "can call its own methods" do
    expect { console.hello }.to println("hello")
  end

  it "can take multiple arguments" do
    expect { console.loglog("hello", "world") }.to println("hello world")
  end

  it "can take a boolean" do
    expect { console.log_if("hello", true) }.to println("hello")
    expect { console.log_if("world", false) }.to_not print
  end

  it "can return a string" do
    expect(console.colorize("hello")).to eq("hello".colorize(:red))
  end

  it "can return an array of strings" do
    expect(console.colorize_lines(["hello", "world"])).to eq(["hello".colorize(:red), "world".colorize(:red)])
  end

  it "can return a boolean" do
    expect(console.is_red("hello")).to eq(false)
    expect(console.is_red("hello".colorize(:red))).to eq(true)
  end

  it "can take a regex" do
    expect { console.log_regex(/[a-z]\w+/im) }.to println('(?mi-x:[a-z]\w+)')
  end

  it "can return a regex" do
    expect(console.filter("testing")).to eq(/LOG[(testing)?]:\s+/)
  end

  [:raise, :raise_panic, :panic].each do |method|
    it "can handle #{method}" do
      expect { console.send(method) }.to raise_error(RuntimeError, "raised from Rust with `#{method}`")
      # Do it twice to make sure we cleaned up correctly the first time
      expect { console.send(method) }.to raise_error(RuntimeError, "raised from Rust with `#{method}`")
    end
  end

  it "can handle invalid arguments" do
    expect { console.log(123) }.to raise_error(TypeError, "Expected a UTF-8 String, got 123")
  end
end

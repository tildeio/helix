require "spec_helper"
require "json"

describe "JsonBuilder" do
  let(:builder) { JsonBuilder.new }
  let(:json) { JSON.parse(builder.to_json) }

  it "can add null" do
    builder["foo"] = nil
    expect(json).to eq({ "foo" => nil })
  end

  it "can add booleans" do
    builder["foo"] = true
    builder["bar"] = false
    expect(json).to eq({ "foo" => true, "bar" => false })
  end

  it "can add integers" do
    builder["foo"] = 12345
    builder["bar"] = -1_000_000
    expect(json).to eq({ "foo" => 12345, "bar" => -1_000_000 })
  end

  it "can add floats" do
    builder["foo"] = 1.2345
    builder["bar"] = -1.0

    expect(->{ builder["baz"] = Float::NAN }).to raise_error(TypeError)
    expect(->{ builder["baz"] = Float::INFINITY }).to raise_error(TypeError)

    expect(json).to eq({ "foo" => 1.2345, "bar" => -1.0 })
  end

  it "can add string" do
    builder["foo"] = "FOO"
    builder["bar"] = "BAR"
    expect(json).to eq({ "foo" => "FOO", "bar" => "BAR" })
  end

  it "can add array" do
    foo = builder["foo"] = [nil, true, 12345, 1.2345, "FOO"]
    bar = builder["bar"] = [nil, false, -1_000_000, -1.0, "BAR"]
    expect(json).to eq({ "foo" => foo, "bar" => bar })
  end

  it "can add hash" do
    foo = builder["foo"] = { "nil" => nil, "true" => true, "12345" => 12345, "1.2345" => 1.2345, "FOO" => "FOO" }
    bar = builder["bar"] = { "nil" => nil, "false" => false, "-1_000_000" => -1_000_000, "-1.0" => -1.0, "BAR" => "BAR" }
    expect(json).to eq({ "foo" => foo, "bar" => bar })
  end

  it "can add nested builder" do
    builder["foo"] = JsonBuilder.new.tap { |inner| inner["foo"] = "FOO" }
    builder["bar"] = JsonBuilder.new.tap { |inner| inner["bar"] = "BAR" }
    expect(json).to eq({ "foo" => { "foo" => "FOO" }, "bar" => { "bar" => "BAR" } })
  end

  it "can convert into a hash" do
    builder["foo"] = "FOO"
    builder["bar"] = nil
    expect(builder.to_h).to eq({ "foo" => "FOO", "bar" => nil })
  end

  it "cannot be used once to_json is called" do
    expect(builder.to_json).to eq("{}")

    expect(->{ builder["foo"] = nil }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_json }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_h }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
  end

  it "cannot be used once to_h is called" do
    expect(builder.to_h).to eq({})

    expect(->{ builder["foo"] = nil }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_json }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_h }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
  end

  it "cannot be used once added to another builder" do
    JsonBuilder.new["foo"] = builder

    expect(->{ builder["foo"] = nil }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_json }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
    expect(->{ builder.to_h }).to raise_error(RuntimeError, "Uninitialized JsonBuilder")
  end
end

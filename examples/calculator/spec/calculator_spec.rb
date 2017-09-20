require "spec_helper"

describe Calculator do
  it "can add numbers" do
    expect(Calculator.add(1.23, -4.56)).to eq(1.23 - 4.56)
    expect(Calculator.add(3, 5)).to eq(8)
  end

  it "can multiply numbers" do
    expect(Calculator.multiply(1.23, -4.56)).to eq(1.23 * -4.56)
    expect(Calculator.multiply(3, 5)).to eq(15)
  end

  it "can divide numbers" do
    expect(Calculator.divide(1.23, -4.56)).to eq(1.23 / -4.56)
    expect(Calculator.divide(4, 2)).to eq(2)
    expect { Calculator.divide(4, 0) }.to raise_error("Division by zero")
  end
end

describe Adder do
  it "can add numbers" do
    expect(Adder.new(1.23).(-4.56)).to eq(1.23 - 4.56)
    expect(Adder.new(3).(5)).to eq(8)
  end
end

describe Multiplier do
  it "can multiply numbers" do
    expect(Multiplier.new(1.23).(-4.56)).to eq(1.23 * -4.56)
    expect(Multiplier.new(3).(5)).to eq(15)
  end
end

describe Divider do
  it "can divide numbers" do
    expect(Divider.new(1.23).(-4.56)).to eq(1.23 / -4.56)
    expect(Divider.new(4).(2)).to eq(2)
    expect { Divider.new(4).(0) }.to raise_error("Division by zero")
  end
end

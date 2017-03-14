require "spec_helper"

describe "Calculator" do
  let(:calculator) { Calculator.new }

  it "can multiply numbers" do
    expect(calculator.multiply(1.23, -4.56)).to eq(-5.6088)
    expect(calculator.multiply(3, 5)).to eq(15)
  end

  it "can add complex" do
    expect(calculator.add_complex(Complex(-2, 9), Complex(-9, 2))).to eq(Complex(-11, 11))
    expect(calculator.add_complex(Complex(-2.1, 9.1), Complex(-9.1, 2.1))).to eq(Complex(-11.2, 11.2))
  end

end

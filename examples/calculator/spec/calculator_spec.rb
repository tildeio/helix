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

  it "can add rational" do
    expect(calculator.add_rational(Rational(1,2), Rational(1,4))).to eq(Rational(3,4))
  end

  it "can operate on generic numbers" do
    expect(calculator.add(1, 2.4)).to eq(3.4)
    expect(calculator.sub(1, 2.4)).to eq(-1.4)
    # Round because floats
    expect(calculator.mul(3, 2.4).round(1)).to eq(7.2)
    expect(calculator.div(6, 2.4)).to eq(2.5)

    rational = calculator.div(Rational(5, 2), 2)
    expect(rational).to eq(Rational(5, 4))
    expect(rational).to be_a(Rational)
  end

end

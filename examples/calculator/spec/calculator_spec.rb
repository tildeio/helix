require "spec_helper"

describe "Calculator" do
  let(:calculator) { Calculator.new }

  it "can multiply numbers" do
    expect(calculator.multiply(1.23, -4.56)).to eq(-5.6088)
    expect(calculator.multiply(3, 5)).to eq(15)
  end

end

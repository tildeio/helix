require "spec_helper"

describe Primitive do
  describe "#is_bool" do
    cases = {
      true => true,
      false => true,
      nil => false,
      "true" => false,
      "" => false,
      0 => false,
      0.5 => false,
      {} => false,
      Object.new => false
    }

    cases.each do |test, expected|
      it "#{test.inspect} => #{expected}" do
        expect(Primitive.is_bool(test)).to eq(expected)
      end
    end
  end
end

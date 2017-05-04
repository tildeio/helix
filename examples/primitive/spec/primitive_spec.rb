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

  describe "#as_bool" do
    it "true" do
      expect(Primitive.as_bool(true)).to eq(true)
    end

    it "nil" do
      expect(-> { Primitive.as_bool(nil) }).to raise_error(RuntimeError)
    end
  end

  describe "#first" do
    it "[1, 2, 3]" do
      expect(Primitive.first([1,2,3])).to eq(1)
    end

    it "['a', 2, 'c']" do
      expect(Primitive.first(['a', 2, 'c'])).to eq('a')
    end
  end
end

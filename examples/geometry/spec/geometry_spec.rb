require "spec_helper"

describe Point do
  let(:origin) { Point.new(0,0) }

  let(:north) { Point.new(0,1) }
  let(:east) { Point.new(1,0) }
  let(:south) { Point.new(0,-1) }
  let(:west) { Point.new(-1,0) }

  it "has the right coordinates" do
    expect(origin.x).to eq(0)
    expect(origin.y).to eq(0)

    expect(north.x).to eq(0)
    expect(north.y).to eq(1)

    expect(east.x).to eq(1)
    expect(east.y).to eq(0)

    expect(south.x).to eq(0)
    expect(south.y).to eq(-1)

    expect(west.x).to eq(-1)
    expect(west.y).to eq(0)
  end

  it "can be turned into an array" do
    expect(origin.to_a).to eq([0,0])
    expect(north.to_a).to eq([0,1])
    expect(east.to_a).to eq([1,0])
    expect(south.to_a).to eq([0,-1])
    expect(west.to_a).to eq([-1,0])
  end
end

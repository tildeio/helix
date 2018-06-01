require "spec_helper"

describe ClassContainingNonClonableFields do
  it "works" do
    c = ClassContainingNonClonableFields.new

    expect(c).to be_instance_of(ClassContainingNonClonableFields)
    expect(c.to_s).to eq("ClassContainingNonClonableFields")
  end
end

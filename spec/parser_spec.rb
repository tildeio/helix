require 'spec_helper'

describe TurboRuby::Parser do
  let(:contents) { File.read(File.expand_path('../fixtures/fast_blank.trb', __FILE__)) }

  it "parses a simple file" do
    ast = subject.parse(contents)

    expect(ast.length).to eq(1)

    klass = ast.first

    expect(klass.name).to eq("String")
    expect(klass.methods.length).to eq(1)

    meth = klass.methods[0]

    expect(meth.name).to eq("blank?")
    expect(meth.arguments.length).to eq(0)
    expect(meth.return_type).to eq("bool")
    expect(meth.body).to eq(["    self.chars().all(|c| c.is_whitespace())\n"])
  end
end

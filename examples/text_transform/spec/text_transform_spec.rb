require "spec_helper"

describe "TextTransform" do
  it "can widen text" do
    expect(TextTransform.widen("Hello Aaron (@tenderlove)!")).to eq("Ｈｅｌｌｏ　Ａａｒｏｎ　（＠ｔｅｎｄｅｒｌｏｖｅ）！")
  end

  it "can widen array" do
    expect(TextTransform.widen_array(%w"Hello Aaron (@tenderlove)!")).to eq(%w"Ｈｅｌｌｏ Ａａｒｏｎ （＠ｔｅｎｄｅｒｌｏｖｅ）！")
  end

  it "can widen hash" do
    expect(TextTransform.widen_hash({
      message: "Hello",
      name: "Aaron",
      handle: "@tenderlove"
    })).to eq({
      "ｍｅｓｓａｇｅ": "Ｈｅｌｌｏ",
      "ｎａｍｅ": "Ａａｒｏｎ",
      "ｈａｎｄｌｅ": "＠ｔｅｎｄｅｒｌｏｖｅ"
    })
  end

  it "can narrowen text" do
    expect(TextTransform.narrowen("Ｈｅｌｌｏ　Ａａｒｏｎ　（＠ｔｅｎｄｅｒｌｏｖｅ）！")).to eq("Hello Aaron (@tenderlove)!")
  end

  it "can narrowen array" do
    expect(TextTransform.narrowen_array(%w"Ｈｅｌｌｏ Ａａｒｏｎ （＠ｔｅｎｄｅｒｌｏｖｅ）！")).to eq(%w"Hello Aaron (@tenderlove)!")
  end

  it "can narrowen hash" do
    expect(TextTransform.narrowen_hash({
      "ｍｅｓｓａｇｅ": "Ｈｅｌｌｏ",
      "ｎａｍｅ": "Ａａｒｏｎ",
      "ｈａｎｄｌｅ": "＠ｔｅｎｄｅｒｌｏｖｅ"
    })).to eq({
      message: "Hello",
      name: "Aaron",
      handle: "@tenderlove"
    })
  end

  it "can flip text" do
    expect(TextTransform.flip("Hello Aaron (@tenderlove)!")).to eq("¡(ǝʌolɹǝpuǝʇ@) uoɹɐ∀ ollǝH")
  end

  it "can flip array" do
    expect(TextTransform.flip_array(%w"Hello Aaron (@tenderlove)!")).to eq(%w"¡(ǝʌolɹǝpuǝʇ@) uoɹɐ∀ ollǝH")
  end

  it "can flip hash" do
    expect(TextTransform.flip_hash({
      message: "Hello",
      name: "Aaron",
      handle: "@tenderlove"
    })).to eq({
      "ollǝH": "ǝƃɐssǝɯ",
      "uoɹɐ∀": "ǝɯɐu",
      "ǝʌolɹǝpuǝʇ@": "ǝlpuɐɥ"
    })
  end
end

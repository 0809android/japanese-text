use japanese_text::*;

fn main() {
    println!("=== japanese-text デモ ===\n");

    // 全角→半角変換
    println!("【全角→半角変換】");
    let input1 = "ＡＢＣ１２３";
    let output1 = to_half_width(input1);
    println!("  入力:  {}", input1);
    println!("  出力:  {}\n", output1);

    // 半角→全角変換
    println!("【半角→全角変換】");
    let input2 = "Hello World 123";
    let output2 = to_full_width(input2);
    println!("  入力:  {}", input2);
    println!("  出力:  {}\n", output2);

    // カタカナ→ひらがな変換
    println!("【カタカナ→ひらがな変換】");
    let input3 = "カタカナ";
    let output3 = to_hiragana(input3);
    println!("  入力:  {}", input3);
    println!("  出力:  {}\n", output3);

    // ひらがな→カタカナ変換
    println!("【ひらがな→カタカナ変換】");
    let input4 = "こんにちは";
    let output4 = to_katakana(input4);
    println!("  入力:  {}", input4);
    println!("  出力:  {}\n", output4);

    // 半角カタカナ→全角カタカナ変換
    println!("【半角カタカナ→全角カタカナ変換】");
    let input5 = "ｶﾀｶﾅ ｶﾞｷﾞｸﾞｹﾞｺﾞ ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ";
    let output5 = half_width_katakana_to_full_width(input5);
    println!("  入力:  {}", input5);
    println!("  出力:  {}\n", output5);

    // 文字種判定
    println!("【文字種判定】");
    println!("  'あ' はひらがな: {}", is_hiragana('あ'));
    println!("  'ア' はカタカナ: {}", is_katakana('ア'));
    println!("  'ｱ' は半角カタカナ: {}", is_half_width_katakana('ｱ'));
    println!("  '漢' は漢字: {}", is_kanji('漢'));
    println!("  'Ａ' は全角文字: {}\n", is_full_width('Ａ'));

    // 文字種カウント
    println!("【文字種カウント】");
    let text = "あいうアイウ漢字ABC123ｱｲｳ";
    let counts = count_character_types(text);
    println!("  テキスト: {}", text);
    println!("  ひらがな: {}", counts.hiragana);
    println!("  カタカナ: {}", counts.katakana);
    println!("  半角カタカナ: {}", counts.half_width_katakana);
    println!("  漢字: {}", counts.kanji);
    println!("  ASCII: {}\n", counts.ascii);

    // 空白正規化
    println!("【空白正規化】");
    let input6 = "Hello　　World\t\tTest";
    let output6 = normalize_whitespace(input6);
    println!("  入力:  {}", input6);
    println!("  出力:  {}\n", output6);

    // 長音記号正規化
    println!("【長音記号正規化】");
    let input7 = "コ〜ヒ〜";
    let output7 = normalize_prolonged_sound(input7);
    println!("  入力:  {}", input7);
    println!("  出力:  {}\n", output7);

    // 繰り返し記号展開
    println!("【繰り返し記号展開】");
    let input8 = "いろゝ かゞ トヽキ";
    let output8 = expand_iteration_marks(input8);
    println!("  入力:  {}", input8);
    println!("  出力:  {}\n", output8);

    // 混在コンテンツの処理
    println!("【混在コンテンツの処理】");
    let input9 = "ＡＢＣあいうカタカナ１２３";
    println!("  元の文字列:      {}", input9);
    println!("  半角変換:        {}", to_half_width(input9));
    println!("  ひらがな変換:    {}", to_hiragana(input9));
    println!("  カタカナ変換:    {}", to_katakana(input9));
}

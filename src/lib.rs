//! # japanese-text
//!
//! 日本語テキスト正規化のための軽量なRustライブラリ
//!
//! ## 特徴
//!
//! - 全角⇔半角変換（ASCII文字）
//! - カタカナ⇔ひらがな変換
//! - シンプルでゼロ依存の実装
//!
//! ## 使用例
//!
//! ```
//! use japanese_text::*;
//!
//! // 全角→半角変換
//! assert_eq!(to_half_width("ＡＢＣ１２３"), "ABC123");
//!
//! // 半角→全角変換
//! assert_eq!(to_full_width("ABC123"), "ＡＢＣ１２３");
//!
//! // カタカナ→ひらがな変換
//! assert_eq!(to_hiragana("カタカナ"), "かたかな");
//!
//! // ひらがな→カタカナ変換
//! assert_eq!(to_katakana("ひらがな"), "ヒラガナ");
//! ```

/// 全角ASCII文字を半角に変換します。
///
/// この関数は全角の英数字や記号（U+FF01-U+FF5E）を、
/// 対応する半角ASCII文字（U+0021-U+007E）に変換します。
///
/// # 使用例
///
/// ```
/// use japanese_text::to_half_width;
///
/// assert_eq!(to_half_width("ＡＢＣ"), "ABC");
/// assert_eq!(to_half_width("１２３"), "123");
/// assert_eq!(to_half_width("！＠＃"), "!@#");
/// assert_eq!(to_half_width("Hello　World"), "Hello World");
/// ```
pub fn to_half_width(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                // Full-width space (U+3000) to half-width space
                '　' => ' ',
                // Full-width ASCII variants (U+FF01-U+FF5E) to half-width
                '\u{FF01}'..='\u{FF5E}' => {
                    char::from_u32(c as u32 - 0xFF01 + 0x0021).unwrap_or(c)
                }
                // Keep other characters as-is
                _ => c,
            }
        })
        .collect()
}

/// 半角ASCII文字を全角に変換します。
///
/// この関数は半角ASCII文字（U+0021-U+007E）を、
/// 対応する全角文字（U+FF01-U+FF5E）に変換します。
///
/// # 使用例
///
/// ```
/// use japanese_text::to_full_width;
///
/// assert_eq!(to_full_width("ABC"), "ＡＢＣ");
/// assert_eq!(to_full_width("123"), "１２３");
/// assert_eq!(to_full_width("!@#"), "！＠＃");
/// assert_eq!(to_full_width("Hello World"), "Ｈｅｌｌｏ　Ｗｏｒｌｄ");
/// ```
pub fn to_full_width(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                // Half-width space to full-width space (U+3000)
                ' ' => '　',
                // Half-width ASCII (U+0021-U+007E) to full-width
                '\u{0021}'..='\u{007E}' => {
                    char::from_u32(c as u32 - 0x0021 + 0xFF01).unwrap_or(c)
                }
                // Keep other characters as-is
                _ => c,
            }
        })
        .collect()
}

/// カタカナをひらがなに変換します。
///
/// この関数はカタカナ文字（U+30A1-U+30F6）を、
/// 対応するひらがな文字（U+3041-U+3096）に変換します。
///
/// # 使用例
///
/// ```
/// use japanese_text::to_hiragana;
///
/// assert_eq!(to_hiragana("カタカナ"), "かたかな");
/// assert_eq!(to_hiragana("コンニチハ"), "こんにちは");
/// assert_eq!(to_hiragana("ヴァイオリン"), "ゔぁいおりん");
/// ```
pub fn to_hiragana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                // Katakana (U+30A1-U+30F6) to Hiragana (U+3041-U+3096)
                '\u{30A1}'..='\u{30F6}' => {
                    char::from_u32(c as u32 - 0x30A1 + 0x3041).unwrap_or(c)
                }
                // Keep other characters as-is
                _ => c,
            }
        })
        .collect()
}

/// ひらがなをカタカナに変換します。
///
/// この関数はひらがな文字（U+3041-U+3096）を、
/// 対応するカタカナ文字（U+30A1-U+30F6）に変換します。
///
/// # 使用例
///
/// ```
/// use japanese_text::to_katakana;
///
/// assert_eq!(to_katakana("ひらがな"), "ヒラガナ");
/// assert_eq!(to_katakana("こんにちは"), "コンニチハ");
/// assert_eq!(to_katakana("ゔぁいおりん"), "ヴァイオリン");
/// ```
pub fn to_katakana(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                // Hiragana (U+3041-U+3096) to Katakana (U+30A1-U+30F6)
                '\u{3041}'..='\u{3096}' => {
                    char::from_u32(c as u32 - 0x3041 + 0x30A1).unwrap_or(c)
                }
                // Keep other characters as-is
                _ => c,
            }
        })
        .collect()
}

/// 文字がひらがなかどうかを判定します。
///
/// # 使用例
///
/// ```
/// use japanese_text::is_hiragana;
///
/// assert_eq!(is_hiragana('あ'), true);
/// assert_eq!(is_hiragana('ア'), false);
/// assert_eq!(is_hiragana('A'), false);
/// ```
pub fn is_hiragana(c: char) -> bool {
    matches!(c, '\u{3041}'..='\u{3096}')
}

/// 文字がカタカナかどうかを判定します。
///
/// # 使用例
///
/// ```
/// use japanese_text::is_katakana;
///
/// assert_eq!(is_katakana('ア'), true);
/// assert_eq!(is_katakana('あ'), false);
/// assert_eq!(is_katakana('A'), false);
/// ```
pub fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A1}'..='\u{30F6}')
}

/// 文字が半角カタカナかどうかを判定します。
///
/// # 使用例
///
/// ```
/// use japanese_text::is_half_width_katakana;
///
/// assert_eq!(is_half_width_katakana('ｱ'), true);
/// assert_eq!(is_half_width_katakana('ア'), false);
/// assert_eq!(is_half_width_katakana('A'), false);
/// ```
pub fn is_half_width_katakana(c: char) -> bool {
    matches!(c, '\u{FF61}'..='\u{FF9F}')
}

/// 文字が漢字（CJK統合漢字）かどうかを判定します。
///
/// # 使用例
///
/// ```
/// use japanese_text::is_kanji;
///
/// assert_eq!(is_kanji('漢'), true);
/// assert_eq!(is_kanji('字'), true);
/// assert_eq!(is_kanji('あ'), false);
/// assert_eq!(is_kanji('A'), false);
/// ```
pub fn is_kanji(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}')
}

/// 文字が全角文字かどうかを判定します。
///
/// # 使用例
///
/// ```
/// use japanese_text::is_full_width;
///
/// assert_eq!(is_full_width('Ａ'), true);
/// assert_eq!(is_full_width('１'), true);
/// assert_eq!(is_full_width('A'), false);
/// ```
pub fn is_full_width(c: char) -> bool {
    matches!(c, '\u{FF01}'..='\u{FF5E}' | '　')
}

/// 文字列内の各文字種の数をカウントします。
///
/// # 使用例
///
/// ```
/// use japanese_text::count_character_types;
///
/// let counts = count_character_types("あア漢ABC123");
/// assert_eq!(counts.hiragana, 1);
/// assert_eq!(counts.katakana, 1);
/// assert_eq!(counts.kanji, 1);
/// assert_eq!(counts.ascii, 6);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterTypes {
    pub hiragana: usize,
    pub katakana: usize,
    pub half_width_katakana: usize,
    pub kanji: usize,
    pub ascii: usize,
    pub full_width: usize,
    pub other: usize,
}

pub fn count_character_types(input: &str) -> CharacterTypes {
    let mut counts = CharacterTypes {
        hiragana: 0,
        katakana: 0,
        half_width_katakana: 0,
        kanji: 0,
        ascii: 0,
        full_width: 0,
        other: 0,
    };

    for c in input.chars() {
        if is_hiragana(c) {
            counts.hiragana += 1;
        } else if is_katakana(c) {
            counts.katakana += 1;
        } else if is_half_width_katakana(c) {
            counts.half_width_katakana += 1;
        } else if is_kanji(c) {
            counts.kanji += 1;
        } else if c.is_ascii() {
            counts.ascii += 1;
        } else if is_full_width(c) {
            counts.full_width += 1;
        } else {
            counts.other += 1;
        }
    }

    counts
}

/// 文字列内の空白文字を正規化します（全角スペース、タブなどを半角スペースに統一）。
///
/// # 使用例
///
/// ```
/// use japanese_text::normalize_whitespace;
///
/// assert_eq!(normalize_whitespace("Hello　World"), "Hello World");
/// assert_eq!(normalize_whitespace("A\t\tB"), "A B");
/// ```
pub fn normalize_whitespace(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_whitespace() || c == '　' {
                ' '
            } else {
                c
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// 半角カタカナを全角カタカナに変換します。
///
/// 濁点（゛）と半濁点（゜）も正しく結合されます。
///
/// # 使用例
///
/// ```
/// use japanese_text::half_width_katakana_to_full_width;
///
/// assert_eq!(half_width_katakana_to_full_width("ｶﾀｶﾅ"), "カタカナ");
/// assert_eq!(half_width_katakana_to_full_width("ｶﾞｷﾞｸﾞｹﾞｺﾞ"), "ガギグゲゴ");
/// assert_eq!(half_width_katakana_to_full_width("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ"), "パピプペポ");
/// ```
pub fn half_width_katakana_to_full_width(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // 次の文字が濁点または半濁点かチェック
        let next = if i + 1 < chars.len() {
            Some(chars[i + 1])
        } else {
            None
        };

        match (c, next) {
            // 濁点付き
            ('ｶ', Some('ﾞ')) => { result.push('ガ'); i += 2; }
            ('ｷ', Some('ﾞ')) => { result.push('ギ'); i += 2; }
            ('ｸ', Some('ﾞ')) => { result.push('グ'); i += 2; }
            ('ｹ', Some('ﾞ')) => { result.push('ゲ'); i += 2; }
            ('ｺ', Some('ﾞ')) => { result.push('ゴ'); i += 2; }
            ('ｻ', Some('ﾞ')) => { result.push('ザ'); i += 2; }
            ('ｼ', Some('ﾞ')) => { result.push('ジ'); i += 2; }
            ('ｽ', Some('ﾞ')) => { result.push('ズ'); i += 2; }
            ('ｾ', Some('ﾞ')) => { result.push('ゼ'); i += 2; }
            ('ｿ', Some('ﾞ')) => { result.push('ゾ'); i += 2; }
            ('ﾀ', Some('ﾞ')) => { result.push('ダ'); i += 2; }
            ('ﾁ', Some('ﾞ')) => { result.push('ヂ'); i += 2; }
            ('ﾂ', Some('ﾞ')) => { result.push('ヅ'); i += 2; }
            ('ﾃ', Some('ﾞ')) => { result.push('デ'); i += 2; }
            ('ﾄ', Some('ﾞ')) => { result.push('ド'); i += 2; }
            ('ﾊ', Some('ﾞ')) => { result.push('バ'); i += 2; }
            ('ﾋ', Some('ﾞ')) => { result.push('ビ'); i += 2; }
            ('ﾌ', Some('ﾞ')) => { result.push('ブ'); i += 2; }
            ('ﾍ', Some('ﾞ')) => { result.push('ベ'); i += 2; }
            ('ﾎ', Some('ﾞ')) => { result.push('ボ'); i += 2; }
            ('ｳ', Some('ﾞ')) => { result.push('ヴ'); i += 2; }

            // 半濁点付き
            ('ﾊ', Some('ﾟ')) => { result.push('パ'); i += 2; }
            ('ﾋ', Some('ﾟ')) => { result.push('ピ'); i += 2; }
            ('ﾌ', Some('ﾟ')) => { result.push('プ'); i += 2; }
            ('ﾍ', Some('ﾟ')) => { result.push('ペ'); i += 2; }
            ('ﾎ', Some('ﾟ')) => { result.push('ポ'); i += 2; }

            // 通常の半角カタカナ
            _ => {
                let full = match c {
                    'ｦ' => 'ヲ', 'ｧ' => 'ァ', 'ｨ' => 'ィ', 'ｩ' => 'ゥ', 'ｪ' => 'ェ', 'ｫ' => 'ォ',
                    'ｬ' => 'ャ', 'ｭ' => 'ュ', 'ｮ' => 'ョ', 'ｯ' => 'ッ', 'ｰ' => 'ー',
                    'ｱ' => 'ア', 'ｲ' => 'イ', 'ｳ' => 'ウ', 'ｴ' => 'エ', 'ｵ' => 'オ',
                    'ｶ' => 'カ', 'ｷ' => 'キ', 'ｸ' => 'ク', 'ｹ' => 'ケ', 'ｺ' => 'コ',
                    'ｻ' => 'サ', 'ｼ' => 'シ', 'ｽ' => 'ス', 'ｾ' => 'セ', 'ｿ' => 'ソ',
                    'ﾀ' => 'タ', 'ﾁ' => 'チ', 'ﾂ' => 'ツ', 'ﾃ' => 'テ', 'ﾄ' => 'ト',
                    'ﾅ' => 'ナ', 'ﾆ' => 'ニ', 'ﾇ' => 'ヌ', 'ﾈ' => 'ネ', 'ﾉ' => 'ノ',
                    'ﾊ' => 'ハ', 'ﾋ' => 'ヒ', 'ﾌ' => 'フ', 'ﾍ' => 'ヘ', 'ﾎ' => 'ホ',
                    'ﾏ' => 'マ', 'ﾐ' => 'ミ', 'ﾑ' => 'ム', 'ﾒ' => 'メ', 'ﾓ' => 'モ',
                    'ﾔ' => 'ヤ', 'ﾕ' => 'ユ', 'ﾖ' => 'ヨ',
                    'ﾗ' => 'ラ', 'ﾘ' => 'リ', 'ﾙ' => 'ル', 'ﾚ' => 'レ', 'ﾛ' => 'ロ',
                    'ﾜ' => 'ワ', 'ﾝ' => 'ン',
                    '｡' => '。', '｢' => '「', '｣' => '」', '､' => '、', '･' => '・',
                    _ => c,
                };
                result.push(full);
                i += 1;
            }
        }
    }

    result
}

/// 長音記号を正規化します（ー、〜、～などを統一）。
///
/// # 使用例
///
/// ```
/// use japanese_text::normalize_prolonged_sound;
///
/// assert_eq!(normalize_prolonged_sound("コーヒー"), "コーヒー");
/// assert_eq!(normalize_prolonged_sound("コ〜ヒ〜"), "コーヒー");
/// ```
pub fn normalize_prolonged_sound(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '〜' | '～' => 'ー',
            _ => c,
        })
        .collect()
}

/// 繰り返し記号を展開します。
///
/// ひらがな・カタカナの繰り返し記号（ゝ、ゞ、ヽ、ヾ）を実際の文字に展開します。
///
/// # 使用例
///
/// ```
/// use japanese_text::expand_iteration_marks;
///
/// assert_eq!(expand_iteration_marks("いろゝ"), "いろろ");
/// assert_eq!(expand_iteration_marks("かゞ"), "かが");
/// ```
pub fn expand_iteration_marks(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut result = String::new();

    for (i, &c) in chars.iter().enumerate() {
        match c {
            // ひらがな繰り返し記号（無声音）
            'ゝ' => {
                if i > 0 {
                    result.push(chars[i - 1]);
                } else {
                    result.push(c);
                }
            }
            // ひらがな繰り返し記号（濁音）
            'ゞ' => {
                if i > 0 {
                    let prev = chars[i - 1];
                    let voiced = add_dakuten(prev);
                    result.push(voiced);
                } else {
                    result.push(c);
                }
            }
            // カタカナ繰り返し記号（無声音）
            'ヽ' => {
                if i > 0 {
                    result.push(chars[i - 1]);
                } else {
                    result.push(c);
                }
            }
            // カタカナ繰り返し記号（濁音）
            'ヾ' => {
                if i > 0 {
                    let prev = chars[i - 1];
                    let voiced = add_dakuten(prev);
                    result.push(voiced);
                } else {
                    result.push(c);
                }
            }
            _ => result.push(c),
        }
    }

    result
}

/// 文字に濁点を追加します（内部ヘルパー関数）。
fn add_dakuten(c: char) -> char {
    match c {
        // ひらがな
        'か' => 'が', 'き' => 'ぎ', 'く' => 'ぐ', 'け' => 'げ', 'こ' => 'ご',
        'さ' => 'ざ', 'し' => 'じ', 'す' => 'ず', 'せ' => 'ぜ', 'そ' => 'ぞ',
        'た' => 'だ', 'ち' => 'ぢ', 'つ' => 'づ', 'て' => 'で', 'と' => 'ど',
        'は' => 'ば', 'ひ' => 'び', 'ふ' => 'ぶ', 'へ' => 'べ', 'ほ' => 'ぼ',
        // カタカナ
        'カ' => 'ガ', 'キ' => 'ギ', 'ク' => 'グ', 'ケ' => 'ゲ', 'コ' => 'ゴ',
        'サ' => 'ザ', 'シ' => 'ジ', 'ス' => 'ズ', 'セ' => 'ゼ', 'ソ' => 'ゾ',
        'タ' => 'ダ', 'チ' => 'ヂ', 'ツ' => 'ヅ', 'テ' => 'デ', 'ト' => 'ド',
        'ハ' => 'バ', 'ヒ' => 'ビ', 'フ' => 'ブ', 'ヘ' => 'ベ', 'ホ' => 'ボ',
        _ => c,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_half_width() {
        assert_eq!(to_half_width("ＡＢＣ"), "ABC");
        assert_eq!(to_half_width("１２３"), "123");
        assert_eq!(to_half_width("！＠＃"), "!@#");
        assert_eq!(to_half_width("　"), " ");
        assert_eq!(to_half_width("Ｈｅｌｌｏ　Ｗｏｒｌｄ"), "Hello World");
        // Mixed content
        assert_eq!(to_half_width("ＡＢＣあいう"), "ABCあいう");
    }

    #[test]
    fn test_to_full_width() {
        assert_eq!(to_full_width("ABC"), "ＡＢＣ");
        assert_eq!(to_full_width("123"), "１２３");
        assert_eq!(to_full_width("!@#"), "！＠＃");
        assert_eq!(to_full_width(" "), "　");
        assert_eq!(to_full_width("Hello World"), "Ｈｅｌｌｏ　Ｗｏｒｌｄ");
        // Mixed content
        assert_eq!(to_full_width("ABCあいう"), "ＡＢＣあいう");
    }

    #[test]
    fn test_to_hiragana() {
        assert_eq!(to_hiragana("カタカナ"), "かたかな");
        assert_eq!(to_hiragana("コンニチハ"), "こんにちは");
        assert_eq!(to_hiragana("アイウエオ"), "あいうえお");
        assert_eq!(to_hiragana("ヴァイオリン"), "ゔぁいおりん");
        // Mixed content
        assert_eq!(to_hiragana("カタカナABC"), "かたかなABC");
    }

    #[test]
    fn test_to_katakana() {
        assert_eq!(to_katakana("ひらがな"), "ヒラガナ");
        assert_eq!(to_katakana("こんにちは"), "コンニチハ");
        assert_eq!(to_katakana("あいうえお"), "アイウエオ");
        assert_eq!(to_katakana("ゔぁいおりん"), "ヴァイオリン");
        // Mixed content
        assert_eq!(to_katakana("ひらがなABC"), "ヒラガナABC");
    }

    #[test]
    fn test_roundtrip_full_half_width() {
        let original = "ABC123!@#";
        let full = to_full_width(original);
        let back = to_half_width(&full);
        assert_eq!(original, back);
    }

    #[test]
    fn test_roundtrip_hiragana_katakana() {
        let original = "こんにちは";
        let katakana = to_katakana(original);
        let back = to_hiragana(&katakana);
        assert_eq!(original, back);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(to_half_width(""), "");
        assert_eq!(to_full_width(""), "");
        assert_eq!(to_hiragana(""), "");
        assert_eq!(to_katakana(""), "");
    }

    #[test]
    fn test_is_hiragana() {
        assert_eq!(is_hiragana('あ'), true);
        assert_eq!(is_hiragana('ん'), true);
        assert_eq!(is_hiragana('ア'), false);
        assert_eq!(is_hiragana('A'), false);
        assert_eq!(is_hiragana('漢'), false);
    }

    #[test]
    fn test_is_katakana() {
        assert_eq!(is_katakana('ア'), true);
        assert_eq!(is_katakana('ン'), true);
        assert_eq!(is_katakana('あ'), false);
        assert_eq!(is_katakana('A'), false);
    }

    #[test]
    fn test_is_half_width_katakana() {
        assert_eq!(is_half_width_katakana('ｱ'), true);
        assert_eq!(is_half_width_katakana('ﾝ'), true);
        assert_eq!(is_half_width_katakana('ア'), false);
        assert_eq!(is_half_width_katakana('A'), false);
    }

    #[test]
    fn test_is_kanji() {
        assert_eq!(is_kanji('漢'), true);
        assert_eq!(is_kanji('字'), true);
        assert_eq!(is_kanji('あ'), false);
        assert_eq!(is_kanji('A'), false);
    }

    #[test]
    fn test_is_full_width() {
        assert_eq!(is_full_width('Ａ'), true);
        assert_eq!(is_full_width('１'), true);
        assert_eq!(is_full_width('　'), true);
        assert_eq!(is_full_width('A'), false);
    }

    #[test]
    fn test_count_character_types() {
        let counts = count_character_types("あア漢ABC123ｱｲｳ");
        assert_eq!(counts.hiragana, 1);
        assert_eq!(counts.katakana, 1);
        assert_eq!(counts.kanji, 1);
        assert_eq!(counts.ascii, 6);
        assert_eq!(counts.half_width_katakana, 3);
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("Hello　World"), "Hello World");
        assert_eq!(normalize_whitespace("A\t\t\tB"), "A B");
        assert_eq!(normalize_whitespace("  Multiple   Spaces  "), "Multiple Spaces");
    }

    #[test]
    fn test_half_width_katakana_to_full_width() {
        assert_eq!(half_width_katakana_to_full_width("ｶﾀｶﾅ"), "カタカナ");
        assert_eq!(half_width_katakana_to_full_width("ｶﾞｷﾞｸﾞｹﾞｺﾞ"), "ガギグゲゴ");
        assert_eq!(half_width_katakana_to_full_width("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ"), "パピプペポ");
        assert_eq!(half_width_katakana_to_full_width("ｺﾝﾆﾁﾊ"), "コンニチハ");
    }

    #[test]
    fn test_normalize_prolonged_sound() {
        assert_eq!(normalize_prolonged_sound("コーヒー"), "コーヒー");
        assert_eq!(normalize_prolonged_sound("コ〜ヒ〜"), "コーヒー");
        assert_eq!(normalize_prolonged_sound("ラーメン"), "ラーメン");
    }

    #[test]
    fn test_expand_iteration_marks() {
        assert_eq!(expand_iteration_marks("いろゝ"), "いろろ");
        assert_eq!(expand_iteration_marks("かゞ"), "かが");
        assert_eq!(expand_iteration_marks("トヽキ"), "トトキ");
        assert_eq!(expand_iteration_marks("カヾ"), "カガ");
    }
}

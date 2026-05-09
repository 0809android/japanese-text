//! # japanese-text
//!
//! 日本語テキスト正規化のための軽量なRustライブラリ
//!
//! ## 特徴
//!
//! - 全角⇔半角変換（ASCII文字）
//! - カタカナ⇔ひらがな変換
//! - Unicode正規化と日本語向け正規化をまとめて適用できるAPI
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

use unicode_normalization::UnicodeNormalization;

/// Unicode正規化形式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnicodeNormalizationForm {
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
}

/// 空白正規化の方式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhitespaceMode {
    Preserve,
    Collapse,
    Trim,
}

/// 一括正規化のオプション。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizeOptions {
    pub unicode: Option<UnicodeNormalizationForm>,
    pub half_width_ascii: bool,
    pub full_width_ascii: bool,
    pub hiragana: bool,
    pub katakana: bool,
    pub half_width_katakana: bool,
    pub full_width_katakana: bool,
    pub combine_dakuten: bool,
    pub decompose_dakuten: bool,
    pub punctuation: bool,
    pub brackets: bool,
    pub symbols: bool,
    pub old_kanji: bool,
    pub remove_variation_selectors: bool,
    pub expand_iteration_marks: bool,
    pub whitespace: WhitespaceMode,
    pub preserve_ascii_tokens: bool,
}

impl Default for NormalizeOptions {
    fn default() -> Self {
        Self {
            unicode: None,
            half_width_ascii: true,
            full_width_ascii: false,
            hiragana: false,
            katakana: false,
            half_width_katakana: true,
            full_width_katakana: false,
            combine_dakuten: true,
            decompose_dakuten: false,
            punctuation: true,
            brackets: true,
            symbols: true,
            old_kanji: true,
            remove_variation_selectors: true,
            expand_iteration_marks: true,
            whitespace: WhitespaceMode::Collapse,
            preserve_ascii_tokens: false,
        }
    }
}

/// 複数の正規化処理をまとめて適用するビルダー。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Normalizer {
    options: NormalizeOptions,
}

impl Normalizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: NormalizeOptions) -> Self {
        Self { options }
    }

    pub fn unicode(mut self, form: UnicodeNormalizationForm) -> Self {
        self.options.unicode = Some(form);
        self
    }

    pub fn unicode_normalization(mut self, form: Option<UnicodeNormalizationForm>) -> Self {
        self.options.unicode = form;
        self
    }

    pub fn half_width_ascii(mut self, enabled: bool) -> Self {
        self.options.half_width_ascii = enabled;
        if enabled {
            self.options.full_width_ascii = false;
        }
        self
    }

    pub fn full_width_ascii(mut self, enabled: bool) -> Self {
        self.options.full_width_ascii = enabled;
        if enabled {
            self.options.half_width_ascii = false;
        }
        self
    }

    pub fn hiragana(mut self, enabled: bool) -> Self {
        self.options.hiragana = enabled;
        if enabled {
            self.options.katakana = false;
        }
        self
    }

    pub fn katakana(mut self, enabled: bool) -> Self {
        self.options.katakana = enabled;
        if enabled {
            self.options.hiragana = false;
        }
        self
    }

    pub fn half_width_katakana(mut self, enabled: bool) -> Self {
        self.options.half_width_katakana = enabled;
        if enabled {
            self.options.full_width_katakana = false;
        }
        self
    }

    pub fn full_width_katakana(mut self, enabled: bool) -> Self {
        self.options.full_width_katakana = enabled;
        if enabled {
            self.options.half_width_katakana = false;
        }
        self
    }

    pub fn whitespace(mut self, mode: WhitespaceMode) -> Self {
        self.options.whitespace = mode;
        self
    }

    pub fn combine_dakuten(mut self, enabled: bool) -> Self {
        self.options.combine_dakuten = enabled;
        if enabled {
            self.options.decompose_dakuten = false;
        }
        self
    }

    pub fn decompose_dakuten(mut self, enabled: bool) -> Self {
        self.options.decompose_dakuten = enabled;
        if enabled {
            self.options.combine_dakuten = false;
        }
        self
    }

    pub fn punctuation(mut self, enabled: bool) -> Self {
        self.options.punctuation = enabled;
        self
    }

    pub fn brackets(mut self, enabled: bool) -> Self {
        self.options.brackets = enabled;
        self
    }

    pub fn symbols(mut self, enabled: bool) -> Self {
        self.options.symbols = enabled;
        self
    }

    pub fn old_kanji(mut self, enabled: bool) -> Self {
        self.options.old_kanji = enabled;
        self
    }

    pub fn remove_variation_selectors(mut self, enabled: bool) -> Self {
        self.options.remove_variation_selectors = enabled;
        self
    }

    pub fn expand_iteration_marks(mut self, enabled: bool) -> Self {
        self.options.expand_iteration_marks = enabled;
        self
    }

    pub fn preserve_ascii_tokens(mut self, enabled: bool) -> Self {
        self.options.preserve_ascii_tokens = enabled;
        self
    }

    pub fn options(&self) -> &NormalizeOptions {
        &self.options
    }

    pub fn normalize(&self, input: &str) -> String {
        normalize_with_options(input, &self.options)
    }
}

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
    map_chars(input, |c| match c {
        '　' => ' ',
        '\u{FF01}'..='\u{FF5E}' => shift_char(c, 0xFF01, 0x0021),
        _ => c,
    })
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
    map_chars(input, |c| match c {
        ' ' => '　',
        '\u{0021}'..='\u{007E}' => shift_char(c, 0x0021, 0xFF01),
        _ => c,
    })
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
/// assert_eq!(to_hiragana("ヷヸヹヺ"), "わ\u{3099}ゐ\u{3099}ゑ\u{3099}を\u{3099}");
/// ```
pub fn to_hiragana(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        match c {
            '\u{30A1}'..='\u{30F6}' => result.push(shift_char(c, 0x30A1, 0x3041)),
            'ヷ' => result.push_str("わ\u{3099}"),
            'ヸ' => result.push_str("ゐ\u{3099}"),
            'ヹ' => result.push_str("ゑ\u{3099}"),
            'ヺ' => result.push_str("を\u{3099}"),
            _ => result.push(c),
        }
    }

    result
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
/// assert_eq!(to_katakana("わ\u{3099}ゐ\u{3099}ゑ\u{3099}を\u{3099}"), "ヷヸヹヺ");
/// ```
pub fn to_katakana(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match chars.peek().copied() {
            Some('\u{3099}') => {
                if let Some(voiced) = voiced_hiragana_to_katakana(c) {
                    result.push(voiced);
                    chars.next();
                    continue;
                }
            }
            Some('\u{309A}') => {
                if let Some(semi_voiced) = semi_voiced_hiragana_to_katakana(c) {
                    result.push(semi_voiced);
                    chars.next();
                    continue;
                }
            }
            _ => {}
        }

        match c {
            '\u{3041}'..='\u{3096}' => result.push(shift_char(c, 0x3041, 0x30A1)),
            _ => result.push(c),
        }
    }

    result
}

/// 全角カタカナを半角カタカナに変換します。
///
/// 濁点・半濁点付きのカタカナは、半角カタカナと半角濁点・半濁点の
/// 2文字に分解されます。
///
/// ```
/// use japanese_text::full_width_katakana_to_half_width;
///
/// assert_eq!(full_width_katakana_to_half_width("カタカナ"), "ｶﾀｶﾅ");
/// assert_eq!(full_width_katakana_to_half_width("ガギグ"), "ｶﾞｷﾞｸﾞ");
/// assert_eq!(full_width_katakana_to_half_width("パピプ"), "ﾊﾟﾋﾟﾌﾟ");
/// ```
pub fn full_width_katakana_to_half_width(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        let half = full_width_katakana_char_to_half_width(c);
        if half.is_empty() {
            result.push(c);
        } else {
            result.push_str(half);
        }
    }

    result
}

/// 濁点・半濁点の結合文字を合成済み文字に変換します。
///
/// ```
/// use japanese_text::combine_dakuten;
///
/// assert_eq!(combine_dakuten("か\u{3099}き\u{3099}"), "がぎ");
/// assert_eq!(combine_dakuten("ハ\u{309A}"), "パ");
/// ```
pub fn combine_dakuten(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match chars.peek().copied() {
            Some('\u{3099}') => {
                if let Some(voiced) = compose_dakuten(c) {
                    result.push(voiced);
                    chars.next();
                    continue;
                }
            }
            Some('\u{309A}') => {
                if let Some(semi_voiced) = compose_handakuten(c) {
                    result.push(semi_voiced);
                    chars.next();
                    continue;
                }
            }
            _ => {}
        }

        result.push(c);
    }

    result
}

/// 濁点・半濁点付き文字を基底文字と結合文字に分解します。
///
/// ```
/// use japanese_text::decompose_dakuten;
///
/// assert_eq!(decompose_dakuten("が"), "か\u{3099}");
/// assert_eq!(decompose_dakuten("パ"), "ハ\u{309A}");
/// ```
pub fn decompose_dakuten(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if let Some((base, mark)) = decompose_dakuten_char(c) {
            result.push(base);
            result.push(mark);
        } else {
            result.push(c);
        }
    }

    result
}

/// Unicode NFC正規化を適用します。
pub fn normalize_nfc(input: &str) -> String {
    input.nfc().collect()
}

/// Unicode NFD正規化を適用します。
pub fn normalize_nfd(input: &str) -> String {
    input.nfd().collect()
}

/// Unicode NFKC正規化を適用します。
pub fn normalize_nfkc(input: &str) -> String {
    input.nfkc().collect()
}

/// Unicode NFKD正規化を適用します。
pub fn normalize_nfkd(input: &str) -> String {
    input.nfkd().collect()
}

/// 句読点を日本語表記に統一します。
///
/// ```
/// use japanese_text::normalize_punctuation;
///
/// assert_eq!(normalize_punctuation("A，B．C､D｡"), "A、B。C、D。");
/// ```
pub fn normalize_punctuation(input: &str) -> String {
    map_chars(input, |c| match c {
        '，' | ',' | '､' => '、',
        '．' | '.' | '｡' => '。',
        _ => c,
    })
}

/// 括弧と引用符を日本語表記に統一します。
///
/// ```
/// use japanese_text::normalize_brackets_and_quotes;
///
/// assert_eq!(normalize_brackets_and_quotes("(\"本文\")"), "（「本文」）");
/// ```
pub fn normalize_brackets_and_quotes(input: &str) -> String {
    let mut result = String::new();
    let mut double_quote_open = true;
    let mut single_quote_open = true;

    for c in input.chars() {
        match c {
            '(' | '（' | '[' | '［' => result.push('（'),
            ')' | '）' | ']' | '］' => result.push('）'),
            '"' => {
                result.push(if double_quote_open { '「' } else { '」' });
                double_quote_open = !double_quote_open;
            }
            '“' | '〝' => result.push('「'),
            '”' | '〟' => result.push('」'),
            '\'' => {
                result.push(if single_quote_open { '『' } else { '』' });
                single_quote_open = !single_quote_open;
            }
            '‘' => result.push('『'),
            '’' => result.push('』'),
            _ => result.push(c),
        }
    }

    result
}

/// 長音、波ダッシュ、マイナス、ハイフン類を正規化します。
///
/// ```
/// use japanese_text::normalize_symbols;
///
/// assert_eq!(normalize_symbols("コ〜ヒ～ - − —"), "コーヒー - - -");
/// ```
pub fn normalize_symbols(input: &str) -> String {
    map_chars(input, |c| match c {
        '〜' | '～' => 'ー',
        '‐' | '‑' | '‒' | '–' | '—' | '―' | '−' | '﹣' | '－' => '-',
        _ => c,
    })
}

/// 代表的な旧字体を新字体に変換します。
///
/// ```
/// use japanese_text::old_kanji_to_new;
///
/// assert_eq!(old_kanji_to_new("舊字體の國語"), "旧字体の国語");
/// ```
pub fn old_kanji_to_new(input: &str) -> String {
    map_chars(input, old_kanji_char_to_new)
}

/// 異体字セレクタを削除します。
///
/// ```
/// use japanese_text::remove_variation_selectors;
///
/// assert_eq!(remove_variation_selectors("葛\u{E0100}"), "葛");
/// ```
pub fn remove_variation_selectors(input: &str) -> String {
    input
        .chars()
        .filter(|&c| !is_variation_selector(c))
        .collect()
}

/// 既定オプションでテキストを正規化します。
pub fn normalize(input: &str) -> String {
    normalize_with_options(input, &NormalizeOptions::default())
}

/// 指定したオプションでテキストを正規化します。
pub fn normalize_with_options(input: &str, options: &NormalizeOptions) -> String {
    if options.preserve_ascii_tokens {
        return normalize_preserving_ascii_tokens(input, options);
    }

    normalize_segment(input, options)
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
/// assert_eq!(is_katakana('ー'), true);
/// assert_eq!(is_katakana('ヷ'), true);
/// assert_eq!(is_katakana('あ'), false);
/// assert_eq!(is_katakana('A'), false);
/// ```
pub fn is_katakana(c: char) -> bool {
    matches!(c, '\u{30A1}'..='\u{30FA}' | 'ー')
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
/// assert_eq!(is_half_width_katakana('｡'), false);
/// assert_eq!(is_half_width_katakana('A'), false);
/// ```
pub fn is_half_width_katakana(c: char) -> bool {
    matches!(c, '\u{FF66}'..='\u{FF9F}')
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
/// assert_eq!(is_full_width('ア'), true);
/// assert_eq!(is_full_width('漢'), true);
/// assert_eq!(is_full_width('A'), false);
/// ```
pub fn is_full_width(c: char) -> bool {
    is_hiragana(c)
        || is_katakana(c)
        || is_kanji(c)
        || matches!(
            c,
            '　'
                | '\u{3000}'..='\u{303F}'
                | '\u{30A0}'..='\u{30FF}'
                | '\u{FF01}'..='\u{FF5E}'
                | '\u{FFE0}'..='\u{FFE6}'
        )
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharacterTypes {
    pub hiragana: usize,
    pub katakana: usize,
    pub half_width_katakana: usize,
    pub kanji: usize,
    pub ascii: usize,
    pub full_width: usize,
    pub other: usize,
}

/// 文字種ごとの比率。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterTypeRatios {
    pub hiragana: f64,
    pub katakana: f64,
    pub half_width_katakana: f64,
    pub kanji: f64,
    pub ascii: f64,
    pub full_width: f64,
    pub other: f64,
}

pub fn count_character_types(input: &str) -> CharacterTypes {
    let mut counts = CharacterTypes::default();

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

/// 文字列内の各文字種の比率を計算します。
pub fn character_type_ratios(input: &str) -> CharacterTypeRatios {
    let counts = count_character_types(input);
    let total = input.chars().count() as f64;

    if total == 0.0 {
        return CharacterTypeRatios::default();
    }

    CharacterTypeRatios {
        hiragana: counts.hiragana as f64 / total,
        katakana: counts.katakana as f64 / total,
        half_width_katakana: counts.half_width_katakana as f64 / total,
        kanji: counts.kanji as f64 / total,
        ascii: counts.ascii as f64 / total,
        full_width: counts.full_width as f64 / total,
        other: counts.other as f64 / total,
    }
}

/// 日本語文字が指定した比率以上かを判定します。
pub fn is_mostly_japanese(input: &str, threshold: f64) -> bool {
    let total = input.chars().count();
    if total == 0 {
        return false;
    }

    let counts = count_character_types(input);
    let japanese = counts.hiragana + counts.katakana + counts.half_width_katakana + counts.kanji;
    japanese as f64 / total as f64 >= threshold
}

/// ひらがな・カタカナ・漢字・ASCIIのうち複数種類が混在しているかを判定します。
pub fn has_mixed_scripts(input: &str) -> bool {
    let counts = count_character_types(input);
    [
        counts.hiragana,
        counts.katakana,
        counts.half_width_katakana,
        counts.kanji,
        counts.ascii,
    ]
    .into_iter()
    .filter(|&count| count > 0)
    .count()
        > 1
}

/// 日本語文字だけを抽出します。
pub fn extract_japanese(input: &str) -> String {
    input
        .chars()
        .filter(|&c| is_hiragana(c) || is_katakana(c) || is_half_width_katakana(c) || is_kanji(c))
        .collect()
}

/// ASCII文字だけを抽出します。
pub fn extract_ascii(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii()).collect()
}

/// Unicodeの記号・句読点に分類される文字を削除します。
pub fn remove_symbols(input: &str) -> String {
    input
        .chars()
        .filter(|&c| !is_symbol_or_punctuation(c))
        .collect()
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
    map_chars(input, |c| {
        if c.is_whitespace() || c == '　' {
            ' '
        } else {
            c
        }
    })
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
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let converted = match chars.peek().copied() {
            Some('ﾞ') => voiced_half_width_katakana(c),
            Some('ﾟ') => semi_voiced_half_width_katakana(c),
            _ => None,
        };

        if let Some(full) = converted {
            result.push(full);
            chars.next();
        } else {
            result.push(half_width_katakana_char_to_full_width(c));
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
    map_chars(input, |c| match c {
        '〜' | '～' => 'ー',
        _ => c,
    })
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
    let mut result = String::new();

    for c in input.chars() {
        match c {
            // ひらがな繰り返し記号（無声音）
            'ゝ' => {
                if let Some(prev) = result.chars().last() {
                    result.push(prev);
                } else {
                    result.push(c);
                }
            }
            // ひらがな繰り返し記号（濁音）
            'ゞ' => {
                if let Some(prev) = result.chars().last() {
                    let voiced = add_dakuten(prev);
                    result.push(voiced);
                } else {
                    result.push(c);
                }
            }
            // カタカナ繰り返し記号（無声音）
            'ヽ' => {
                if let Some(prev) = result.chars().last() {
                    result.push(prev);
                } else {
                    result.push(c);
                }
            }
            // カタカナ繰り返し記号（濁音）
            'ヾ' => {
                if let Some(prev) = result.chars().last() {
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

fn normalize_segment(input: &str, options: &NormalizeOptions) -> String {
    let mut text = match options.unicode {
        Some(UnicodeNormalizationForm::Nfc) => normalize_nfc(input),
        Some(UnicodeNormalizationForm::Nfd) => normalize_nfd(input),
        Some(UnicodeNormalizationForm::Nfkc) => normalize_nfkc(input),
        Some(UnicodeNormalizationForm::Nfkd) => normalize_nfkd(input),
        None => input.to_string(),
    };

    if options.remove_variation_selectors {
        text = remove_variation_selectors(&text);
    }
    if options.half_width_katakana {
        text = half_width_katakana_to_full_width(&text);
    }
    if options.hiragana {
        text = to_hiragana(&text);
    }
    if options.katakana {
        text = to_katakana(&text);
    }
    if options.decompose_dakuten {
        text = decompose_dakuten(&text);
    } else if options.combine_dakuten {
        text = combine_dakuten(&text);
    }
    if options.full_width_katakana {
        text = full_width_katakana_to_half_width(&text);
    }
    if options.symbols {
        text = normalize_symbols(&text);
    }
    if options.half_width_ascii {
        text = to_half_width(&text);
    }
    if options.full_width_ascii {
        text = to_full_width(&text);
    }
    if options.punctuation {
        text = normalize_punctuation(&text);
    }
    if options.brackets {
        text = normalize_brackets_and_quotes(&text);
    }
    if options.old_kanji {
        text = old_kanji_to_new(&text);
    }
    if options.expand_iteration_marks {
        text = expand_iteration_marks(&text);
    }

    match options.whitespace {
        WhitespaceMode::Preserve => text,
        WhitespaceMode::Collapse => normalize_whitespace(&text),
        WhitespaceMode::Trim => text.trim().to_string(),
    }
}

fn normalize_preserving_ascii_tokens(input: &str, options: &NormalizeOptions) -> String {
    let mut result = String::new();
    let mut ascii_run = String::new();
    let mut normal_run = String::new();

    for c in input.chars() {
        if c.is_ascii() && !c.is_ascii_whitespace() {
            push_normalized_segment(&mut result, &normal_run, options);
            normal_run.clear();
            ascii_run.push(c);
        } else {
            push_normalized_or_preserved_token(&mut result, &ascii_run, options);
            ascii_run.clear();
            normal_run.push(c);
        }
    }

    push_normalized_or_preserved_token(&mut result, &ascii_run, options);
    push_normalized_segment(&mut result, &normal_run, options);

    match options.whitespace {
        WhitespaceMode::Preserve => result,
        WhitespaceMode::Collapse => normalize_whitespace(&result),
        WhitespaceMode::Trim => result.trim().to_string(),
    }
}

fn push_normalized_segment(result: &mut String, segment: &str, options: &NormalizeOptions) {
    if segment.is_empty() {
        return;
    }

    let mut segment_options = options.clone();
    segment_options.preserve_ascii_tokens = false;
    segment_options.whitespace = WhitespaceMode::Preserve;
    result.push_str(&normalize_segment(segment, &segment_options));
}

fn push_normalized_or_preserved_token(
    result: &mut String,
    token: &str,
    options: &NormalizeOptions,
) {
    if token.is_empty() {
        return;
    }

    if let Some((leading, preserved, trailing)) = split_preserved_ascii_token(token) {
        push_normalized_segment(result, leading, options);
        result.push_str(preserved);
        push_normalized_segment(result, trailing, options);
    } else {
        push_normalized_segment(result, token, options);
    }
}

fn split_preserved_ascii_token(token: &str) -> Option<(&str, &str, &str)> {
    if is_number_like(token) {
        return Some(("", token, ""));
    }

    let leading_start = token
        .char_indices()
        .find(|&(_, c)| !is_ascii_token_leading_delimiter(c))
        .map(|(idx, _)| idx)
        .unwrap_or(token.len());
    let (leading, rest) = token.split_at(leading_start);

    let mut core_end = rest.len();
    while core_end > 0 {
        let mut chars = rest[..core_end].char_indices();
        let Some((idx, c)) = chars.next_back() else {
            break;
        };

        if is_ascii_token_trailing_delimiter(c) {
            core_end = idx;
        } else {
            break;
        }
    }

    let candidate = &rest[..core_end];

    if let Some((preserved_start, preserved_end)) = find_preserved_ascii_core(candidate) {
        let preserved_start = leading.len() + preserved_start;
        let preserved_end = leading.len() + preserved_end;
        Some((
            &token[..preserved_start],
            &token[preserved_start..preserved_end],
            &token[preserved_end..],
        ))
    } else {
        None
    }
}

fn is_url_like(token: &str) -> bool {
    token.starts_with("http://") || token.starts_with("https://")
}

fn find_preserved_ascii_core(token: &str) -> Option<(usize, usize)> {
    if is_url_like(token) || is_email_like(token) || is_number_like(token) {
        return Some((0, token.len()));
    }

    let url_start = match (token.find("http://"), token.find("https://")) {
        (Some(http), Some(https)) => Some(http.min(https)),
        (Some(http), None) => Some(http),
        (None, Some(https)) => Some(https),
        (None, None) => None,
    };
    if let Some(start) = url_start {
        return Some((start, token.len()));
    }

    token
        .char_indices()
        .find_map(|(start, _)| is_email_like(&token[start..]).then_some((start, token.len())))
}

fn is_ascii_token_leading_delimiter(c: char) -> bool {
    matches!(
        c,
        '(' | '[' | '{' | '<' | '"' | '\'' | '（' | '［' | '｛' | '「' | '『'
    )
}

fn is_ascii_token_trailing_delimiter(c: char) -> bool {
    matches!(
        c,
        ')' | ']'
            | '}'
            | '>'
            | '"'
            | '\''
            | ','
            | '.'
            | '，'
            | '．'
            | '、'
            | '。'
            | '）'
            | '］'
            | '｝'
            | '」'
            | '』'
    )
}

fn is_email_like(token: &str) -> bool {
    let Some((local, domain)) = token.split_once('@') else {
        return false;
    };

    !local.is_empty()
        && domain.contains('.')
        && domain.len() >= 3
        && token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '@' | '.' | '_' | '%' | '+' | '-'))
}

fn is_number_like(token: &str) -> bool {
    let mut has_digit = false;

    for c in token.chars() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if !matches!(c, '.' | ',' | ':' | '/' | '-' | '+' | '%' | '_' | '#') {
            return false;
        }
    }

    has_digit
}

fn map_chars(input: &str, convert: impl Fn(char) -> char) -> String {
    input.chars().map(convert).collect()
}

fn shift_char(c: char, from_start: u32, to_start: u32) -> char {
    char::from_u32(c as u32 - from_start + to_start).unwrap_or(c)
}

fn half_width_katakana_char_to_full_width(c: char) -> char {
    match c {
        'ｦ' => 'ヲ',
        'ｧ' => 'ァ',
        'ｨ' => 'ィ',
        'ｩ' => 'ゥ',
        'ｪ' => 'ェ',
        'ｫ' => 'ォ',
        'ｬ' => 'ャ',
        'ｭ' => 'ュ',
        'ｮ' => 'ョ',
        'ｯ' => 'ッ',
        'ｰ' => 'ー',
        'ｱ' => 'ア',
        'ｲ' => 'イ',
        'ｳ' => 'ウ',
        'ｴ' => 'エ',
        'ｵ' => 'オ',
        'ｶ' => 'カ',
        'ｷ' => 'キ',
        'ｸ' => 'ク',
        'ｹ' => 'ケ',
        'ｺ' => 'コ',
        'ｻ' => 'サ',
        'ｼ' => 'シ',
        'ｽ' => 'ス',
        'ｾ' => 'セ',
        'ｿ' => 'ソ',
        'ﾀ' => 'タ',
        'ﾁ' => 'チ',
        'ﾂ' => 'ツ',
        'ﾃ' => 'テ',
        'ﾄ' => 'ト',
        'ﾅ' => 'ナ',
        'ﾆ' => 'ニ',
        'ﾇ' => 'ヌ',
        'ﾈ' => 'ネ',
        'ﾉ' => 'ノ',
        'ﾊ' => 'ハ',
        'ﾋ' => 'ヒ',
        'ﾌ' => 'フ',
        'ﾍ' => 'ヘ',
        'ﾎ' => 'ホ',
        'ﾏ' => 'マ',
        'ﾐ' => 'ミ',
        'ﾑ' => 'ム',
        'ﾒ' => 'メ',
        'ﾓ' => 'モ',
        'ﾔ' => 'ヤ',
        'ﾕ' => 'ユ',
        'ﾖ' => 'ヨ',
        'ﾗ' => 'ラ',
        'ﾘ' => 'リ',
        'ﾙ' => 'ル',
        'ﾚ' => 'レ',
        'ﾛ' => 'ロ',
        'ﾜ' => 'ワ',
        'ﾝ' => 'ン',
        '｡' => '。',
        '｢' => '「',
        '｣' => '」',
        '､' => '、',
        '･' => '・',
        _ => c,
    }
}

fn full_width_katakana_char_to_half_width(c: char) -> &'static str {
    match c {
        'ヲ' => "ｦ",
        'ァ' => "ｧ",
        'ィ' => "ｨ",
        'ゥ' => "ｩ",
        'ェ' => "ｪ",
        'ォ' => "ｫ",
        'ャ' => "ｬ",
        'ュ' => "ｭ",
        'ョ' => "ｮ",
        'ッ' => "ｯ",
        'ー' => "ｰ",
        'ア' => "ｱ",
        'イ' => "ｲ",
        'ウ' => "ｳ",
        'エ' => "ｴ",
        'オ' => "ｵ",
        'カ' => "ｶ",
        'キ' => "ｷ",
        'ク' => "ｸ",
        'ケ' => "ｹ",
        'コ' => "ｺ",
        'サ' => "ｻ",
        'シ' => "ｼ",
        'ス' => "ｽ",
        'セ' => "ｾ",
        'ソ' => "ｿ",
        'タ' => "ﾀ",
        'チ' => "ﾁ",
        'ツ' => "ﾂ",
        'テ' => "ﾃ",
        'ト' => "ﾄ",
        'ナ' => "ﾅ",
        'ニ' => "ﾆ",
        'ヌ' => "ﾇ",
        'ネ' => "ﾈ",
        'ノ' => "ﾉ",
        'ハ' => "ﾊ",
        'ヒ' => "ﾋ",
        'フ' => "ﾌ",
        'ヘ' => "ﾍ",
        'ホ' => "ﾎ",
        'マ' => "ﾏ",
        'ミ' => "ﾐ",
        'ム' => "ﾑ",
        'メ' => "ﾒ",
        'モ' => "ﾓ",
        'ヤ' => "ﾔ",
        'ユ' => "ﾕ",
        'ヨ' => "ﾖ",
        'ラ' => "ﾗ",
        'リ' => "ﾘ",
        'ル' => "ﾙ",
        'レ' => "ﾚ",
        'ロ' => "ﾛ",
        'ワ' => "ﾜ",
        'ン' => "ﾝ",
        'ヷ' => "ﾜﾞ",
        'ヸ' => "ｲﾞ",
        'ヹ' => "ｴﾞ",
        'ヺ' => "ｦﾞ",
        'ガ' => "ｶﾞ",
        'ギ' => "ｷﾞ",
        'グ' => "ｸﾞ",
        'ゲ' => "ｹﾞ",
        'ゴ' => "ｺﾞ",
        'ザ' => "ｻﾞ",
        'ジ' => "ｼﾞ",
        'ズ' => "ｽﾞ",
        'ゼ' => "ｾﾞ",
        'ゾ' => "ｿﾞ",
        'ダ' => "ﾀﾞ",
        'ヂ' => "ﾁﾞ",
        'ヅ' => "ﾂﾞ",
        'デ' => "ﾃﾞ",
        'ド' => "ﾄﾞ",
        'バ' => "ﾊﾞ",
        'ビ' => "ﾋﾞ",
        'ブ' => "ﾌﾞ",
        'ベ' => "ﾍﾞ",
        'ボ' => "ﾎﾞ",
        'ヴ' => "ｳﾞ",
        'パ' => "ﾊﾟ",
        'ピ' => "ﾋﾟ",
        'プ' => "ﾌﾟ",
        'ペ' => "ﾍﾟ",
        'ポ' => "ﾎﾟ",
        '。' => "｡",
        '「' => "｢",
        '」' => "｣",
        '、' => "､",
        '・' => "･",
        _ => "",
    }
}

fn voiced_half_width_katakana(c: char) -> Option<char> {
    Some(match c {
        'ｶ' => 'ガ',
        'ｷ' => 'ギ',
        'ｸ' => 'グ',
        'ｹ' => 'ゲ',
        'ｺ' => 'ゴ',
        'ｻ' => 'ザ',
        'ｼ' => 'ジ',
        'ｽ' => 'ズ',
        'ｾ' => 'ゼ',
        'ｿ' => 'ゾ',
        'ﾀ' => 'ダ',
        'ﾁ' => 'ヂ',
        'ﾂ' => 'ヅ',
        'ﾃ' => 'デ',
        'ﾄ' => 'ド',
        'ﾊ' => 'バ',
        'ﾋ' => 'ビ',
        'ﾌ' => 'ブ',
        'ﾍ' => 'ベ',
        'ﾎ' => 'ボ',
        'ｳ' => 'ヴ',
        'ﾜ' => 'ヷ',
        'ｲ' => 'ヸ',
        'ｴ' => 'ヹ',
        'ｦ' => 'ヺ',
        _ => return None,
    })
}

fn semi_voiced_half_width_katakana(c: char) -> Option<char> {
    Some(match c {
        'ﾊ' => 'パ',
        'ﾋ' => 'ピ',
        'ﾌ' => 'プ',
        'ﾍ' => 'ペ',
        'ﾎ' => 'ポ',
        _ => return None,
    })
}

fn voiced_hiragana_to_katakana(c: char) -> Option<char> {
    Some(match c {
        'か' => 'ガ',
        'き' => 'ギ',
        'く' => 'グ',
        'け' => 'ゲ',
        'こ' => 'ゴ',
        'さ' => 'ザ',
        'し' => 'ジ',
        'す' => 'ズ',
        'せ' => 'ゼ',
        'そ' => 'ゾ',
        'た' => 'ダ',
        'ち' => 'ヂ',
        'つ' => 'ヅ',
        'て' => 'デ',
        'と' => 'ド',
        'は' => 'バ',
        'ひ' => 'ビ',
        'ふ' => 'ブ',
        'へ' => 'ベ',
        'ほ' => 'ボ',
        'う' => 'ヴ',
        'わ' => 'ヷ',
        'ゐ' => 'ヸ',
        'ゑ' => 'ヹ',
        'を' => 'ヺ',
        _ => return None,
    })
}

fn semi_voiced_hiragana_to_katakana(c: char) -> Option<char> {
    Some(match c {
        'は' => 'パ',
        'ひ' => 'ピ',
        'ふ' => 'プ',
        'へ' => 'ペ',
        'ほ' => 'ポ',
        _ => return None,
    })
}

fn compose_dakuten(c: char) -> Option<char> {
    let voiced = add_dakuten(c);
    (voiced != c).then_some(voiced)
}

fn compose_handakuten(c: char) -> Option<char> {
    Some(match c {
        'は' => 'ぱ',
        'ひ' => 'ぴ',
        'ふ' => 'ぷ',
        'へ' => 'ぺ',
        'ほ' => 'ぽ',
        'ハ' => 'パ',
        'ヒ' => 'ピ',
        'フ' => 'プ',
        'ヘ' => 'ペ',
        'ホ' => 'ポ',
        _ => return None,
    })
}

fn decompose_dakuten_char(c: char) -> Option<(char, char)> {
    Some(match c {
        'が' => ('か', '\u{3099}'),
        'ぎ' => ('き', '\u{3099}'),
        'ぐ' => ('く', '\u{3099}'),
        'げ' => ('け', '\u{3099}'),
        'ご' => ('こ', '\u{3099}'),
        'ざ' => ('さ', '\u{3099}'),
        'じ' => ('し', '\u{3099}'),
        'ず' => ('す', '\u{3099}'),
        'ぜ' => ('せ', '\u{3099}'),
        'ぞ' => ('そ', '\u{3099}'),
        'だ' => ('た', '\u{3099}'),
        'ぢ' => ('ち', '\u{3099}'),
        'づ' => ('つ', '\u{3099}'),
        'で' => ('て', '\u{3099}'),
        'ど' => ('と', '\u{3099}'),
        'ば' => ('は', '\u{3099}'),
        'び' => ('ひ', '\u{3099}'),
        'ぶ' => ('ふ', '\u{3099}'),
        'べ' => ('へ', '\u{3099}'),
        'ぼ' => ('ほ', '\u{3099}'),
        'ゔ' => ('う', '\u{3099}'),
        'ぱ' => ('は', '\u{309A}'),
        'ぴ' => ('ひ', '\u{309A}'),
        'ぷ' => ('ふ', '\u{309A}'),
        'ぺ' => ('へ', '\u{309A}'),
        'ぽ' => ('ほ', '\u{309A}'),
        'ガ' => ('カ', '\u{3099}'),
        'ギ' => ('キ', '\u{3099}'),
        'グ' => ('ク', '\u{3099}'),
        'ゲ' => ('ケ', '\u{3099}'),
        'ゴ' => ('コ', '\u{3099}'),
        'ザ' => ('サ', '\u{3099}'),
        'ジ' => ('シ', '\u{3099}'),
        'ズ' => ('ス', '\u{3099}'),
        'ゼ' => ('セ', '\u{3099}'),
        'ゾ' => ('ソ', '\u{3099}'),
        'ダ' => ('タ', '\u{3099}'),
        'ヂ' => ('チ', '\u{3099}'),
        'ヅ' => ('ツ', '\u{3099}'),
        'デ' => ('テ', '\u{3099}'),
        'ド' => ('ト', '\u{3099}'),
        'バ' => ('ハ', '\u{3099}'),
        'ビ' => ('ヒ', '\u{3099}'),
        'ブ' => ('フ', '\u{3099}'),
        'ベ' => ('ヘ', '\u{3099}'),
        'ボ' => ('ホ', '\u{3099}'),
        'ヴ' => ('ウ', '\u{3099}'),
        'ヷ' => ('ワ', '\u{3099}'),
        'ヸ' => ('ヰ', '\u{3099}'),
        'ヹ' => ('ヱ', '\u{3099}'),
        'ヺ' => ('ヲ', '\u{3099}'),
        'パ' => ('ハ', '\u{309A}'),
        'ピ' => ('ヒ', '\u{309A}'),
        'プ' => ('フ', '\u{309A}'),
        'ペ' => ('ヘ', '\u{309A}'),
        'ポ' => ('ホ', '\u{309A}'),
        _ => return None,
    })
}

/// 文字に濁点を追加します（内部ヘルパー関数）。
fn add_dakuten(c: char) -> char {
    match c {
        // ひらがな
        'か' => 'が',
        'き' => 'ぎ',
        'く' => 'ぐ',
        'け' => 'げ',
        'こ' => 'ご',
        'さ' => 'ざ',
        'し' => 'じ',
        'す' => 'ず',
        'せ' => 'ぜ',
        'そ' => 'ぞ',
        'た' => 'だ',
        'ち' => 'ぢ',
        'つ' => 'づ',
        'て' => 'で',
        'と' => 'ど',
        'う' => 'ゔ',
        'は' => 'ば',
        'ひ' => 'び',
        'ふ' => 'ぶ',
        'へ' => 'べ',
        'ほ' => 'ぼ',
        // カタカナ
        'カ' => 'ガ',
        'キ' => 'ギ',
        'ク' => 'グ',
        'ケ' => 'ゲ',
        'コ' => 'ゴ',
        'サ' => 'ザ',
        'シ' => 'ジ',
        'ス' => 'ズ',
        'セ' => 'ゼ',
        'ソ' => 'ゾ',
        'タ' => 'ダ',
        'チ' => 'ヂ',
        'ツ' => 'ヅ',
        'テ' => 'デ',
        'ト' => 'ド',
        'ウ' => 'ヴ',
        'ワ' => 'ヷ',
        'ヰ' => 'ヸ',
        'ヱ' => 'ヹ',
        'ヲ' => 'ヺ',
        'ハ' => 'バ',
        'ヒ' => 'ビ',
        'フ' => 'ブ',
        'ヘ' => 'ベ',
        'ホ' => 'ボ',
        _ => c,
    }
}

fn old_kanji_char_to_new(c: char) -> char {
    match c {
        '亞' => '亜',
        '惡' => '悪',
        '壓' => '圧',
        '圍' => '囲',
        '爲' => '為',
        '醫' => '医',
        '壹' => '壱',
        '稻' => '稲',
        '飮' => '飲',
        '隱' => '隠',
        '營' => '営',
        '榮' => '栄',
        '驛' => '駅',
        '圓' => '円',
        '鹽' => '塩',
        '奧' => '奥',
        '應' => '応',
        '歐' => '欧',
        '毆' => '殴',
        '櫻' => '桜',
        '假' => '仮',
        '價' => '価',
        '畫' => '画',
        '會' => '会',
        '懷' => '懐',
        '壞' => '壊',
        '樂' => '楽',
        '氣' => '気',
        '龜' => '亀',
        '僞' => '偽',
        '舊' => '旧',
        '據' => '拠',
        '擧' => '挙',
        '峽' => '峡',
        '狹' => '狭',
        '區' => '区',
        '驅' => '駆',
        '徑' => '径',
        '莖' => '茎',
        '惠' => '恵',
        '溪' => '渓',
        '經' => '経',
        '繼' => '継',
        '缺' => '欠',
        '劍' => '剣',
        '檢' => '検',
        '權' => '権',
        '獻' => '献',
        '縣' => '県',
        '險' => '険',
        '嚴' => '厳',
        '廣' => '広',
        '鑛' => '鉱',
        '號' => '号',
        '國' => '国',
        '黑' => '黒',
        '濟' => '済',
        '齋' => '斎',
        '劑' => '剤',
        '雜' => '雑',
        '參' => '参',
        '棧' => '桟',
        '蠶' => '蚕',
        '殘' => '残',
        '絲' => '糸',
        '齒' => '歯',
        '兒' => '児',
        '實' => '実',
        '舍' => '舎',
        '寫' => '写',
        '釋' => '釈',
        '壽' => '寿',
        '從' => '従',
        '澁' => '渋',
        '獸' => '獣',
        '縱' => '縦',
        '肅' => '粛',
        '處' => '処',
        '敍' => '叙',
        '將' => '将',
        '稱' => '称',
        '證' => '証',
        '奬' => '奨',
        '條' => '条',
        '乘' => '乗',
        '淨' => '浄',
        '剩' => '剰',
        '疊' => '畳',
        '讓' => '譲',
        '釀' => '醸',
        '眞' => '真',
        '寢' => '寝',
        '愼' => '慎',
        '盡' => '尽',
        '圖' => '図',
        '粹' => '粋',
        '醉' => '酔',
        '穗' => '穂',
        '隨' => '随',
        '髓' => '髄',
        '數' => '数',
        '聲' => '声',
        '靜' => '静',
        '齊' => '斉',
        '攝' => '摂',
        '竊' => '窃',
        '專' => '専',
        '戰' => '戦',
        '淺' => '浅',
        '潛' => '潜',
        '遷' => '遷',
        '踐' => '践',
        '錢' => '銭',
        '禪' => '禅',
        '雙' => '双',
        '壯' => '壮',
        '爭' => '争',
        '莊' => '荘',
        '搜' => '捜',
        '插' => '挿',
        '巢' => '巣',
        '裝' => '装',
        '總' => '総',
        '騷' => '騒',
        '臟' => '臓',
        '藏' => '蔵',
        '屬' => '属',
        '續' => '続',
        '墮' => '堕',
        '對' => '対',
        '體' => '体',
        '帶' => '帯',
        '滯' => '滞',
        '臺' => '台',
        '瀧' => '滝',
        '擇' => '択',
        '澤' => '沢',
        '單' => '単',
        '膽' => '胆',
        '團' => '団',
        '彈' => '弾',
        '遲' => '遅',
        '癡' => '痴',
        '蟲' => '虫',
        '晝' => '昼',
        '鑄' => '鋳',
        '廳' => '庁',
        '聽' => '聴',
        '敕' => '勅',
        '鎭' => '鎮',
        '遞' => '逓',
        '鐵' => '鉄',
        '轉' => '転',
        '傳' => '伝',
        '黨' => '党',
        '盜' => '盗',
        '燈' => '灯',
        '當' => '当',
        '鬪' => '闘',
        '德' => '徳',
        '獨' => '独',
        '讀' => '読',
        '屆' => '届',
        '繩' => '縄',
        '貳' => '弐',
        '惱' => '悩',
        '腦' => '脳',
        '霸' => '覇',
        '廢' => '廃',
        '賣' => '売',
        '發' => '発',
        '髮' => '髪',
        '拔' => '抜',
        '蠻' => '蛮',
        '祕' => '秘',
        '濱' => '浜',
        '拂' => '払',
        '佛' => '仏',
        '竝' => '並',
        '變' => '変',
        '邊' => '辺',
        '辯' => '弁',
        '辨' => '弁',
        '瓣' => '弁',
        '舖' => '舗',
        '寶' => '宝',
        '豐' => '豊',
        '沒' => '没',
        '飜' => '翻',
        '萬' => '万',
        '滿' => '満',
        '默' => '黙',
        '藥' => '薬',
        '譯' => '訳',
        '豫' => '予',
        '餘' => '余',
        '與' => '与',
        '譽' => '誉',
        '搖' => '揺',
        '樣' => '様',
        '謠' => '謡',
        '來' => '来',
        '亂' => '乱',
        '覽' => '覧',
        '龍' => '竜',
        '兩' => '両',
        '獵' => '猟',
        '綠' => '緑',
        '壘' => '塁',
        '禮' => '礼',
        '勞' => '労',
        '樓' => '楼',
        '灣' => '湾',
        _ => c,
    }
}

fn is_variation_selector(c: char) -> bool {
    matches!(c, '\u{FE00}'..='\u{FE0F}' | '\u{E0100}'..='\u{E01EF}')
}

fn is_symbol_or_punctuation(c: char) -> bool {
    !c.is_whitespace()
        && (c.is_ascii_punctuation()
            || matches!(
                c,
                '\u{2000}'..='\u{206F}'
                    | '\u{3000}'..='\u{303F}'
                    | '\u{FE10}'..='\u{FE1F}'
                    | '\u{FE30}'..='\u{FE4F}'
                    | '\u{FF01}'..='\u{FF0F}'
                    | '\u{FF1A}'..='\u{FF20}'
                    | '\u{FF3B}'..='\u{FF40}'
                    | '\u{FF5B}'..='\u{FF65}'
                    | '\u{FFE0}'..='\u{FFE6}'
            )
            || is_japanese_symbol(c))
}

fn is_japanese_symbol(c: char) -> bool {
    matches!(
        c,
        '、' | '。'
            | '・'
            | '「'
            | '」'
            | '『'
            | '』'
            | '（'
            | '）'
            | '［'
            | '］'
            | '【'
            | '】'
            | '〜'
            | '～'
            | '…'
            | '※'
            | '〒'
            | '〆'
            | '〇'
            | '〃'
            | 'ゝ'
            | 'ゞ'
            | 'ヽ'
            | 'ヾ'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
        assert_eq!(
            to_hiragana("ヷヸヹヺ"),
            "わ\u{3099}ゐ\u{3099}ゑ\u{3099}を\u{3099}"
        );
        // Mixed content
        assert_eq!(to_hiragana("カタカナABC"), "かたかなABC");
    }

    #[test]
    fn test_to_katakana() {
        assert_eq!(to_katakana("ひらがな"), "ヒラガナ");
        assert_eq!(to_katakana("こんにちは"), "コンニチハ");
        assert_eq!(to_katakana("あいうえお"), "アイウエオ");
        assert_eq!(to_katakana("ゔぁいおりん"), "ヴァイオリン");
        assert_eq!(
            to_katakana("わ\u{3099}ゐ\u{3099}ゑ\u{3099}を\u{3099}"),
            "ヷヸヹヺ"
        );
        assert_eq!(to_katakana("か\u{3099}は\u{309A}"), "ガパ");
        assert_eq!(to_katakana(&to_hiragana("ヷヸヹヺ")), "ヷヸヹヺ");
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
        assert!(is_hiragana('あ'));
        assert!(is_hiragana('ん'));
        assert!(!is_hiragana('ア'));
        assert!(!is_hiragana('A'));
        assert!(!is_hiragana('漢'));
    }

    #[test]
    fn test_is_katakana() {
        assert!(is_katakana('ア'));
        assert!(is_katakana('ン'));
        assert!(is_katakana('ー'));
        assert!(is_katakana('ヷ'));
        assert!(is_katakana('ヸ'));
        assert!(is_katakana('ヹ'));
        assert!(is_katakana('ヺ'));
        assert!(!is_katakana('あ'));
        assert!(!is_katakana('A'));
    }

    #[test]
    fn test_is_half_width_katakana() {
        assert!(is_half_width_katakana('ｱ'));
        assert!(is_half_width_katakana('ﾝ'));
        assert!(is_half_width_katakana('ﾞ'));
        assert!(is_half_width_katakana('ﾟ'));
        assert!(!is_half_width_katakana('｡'));
        assert!(!is_half_width_katakana('｢'));
        assert!(!is_half_width_katakana('､'));
        assert!(!is_half_width_katakana('ア'));
        assert!(!is_half_width_katakana('A'));
    }

    #[test]
    fn test_is_kanji() {
        assert!(is_kanji('漢'));
        assert!(is_kanji('字'));
        assert!(!is_kanji('あ'));
        assert!(!is_kanji('A'));
    }

    #[test]
    fn test_is_full_width() {
        assert!(is_full_width('Ａ'));
        assert!(is_full_width('１'));
        assert!(is_full_width('ア'));
        assert!(is_full_width('あ'));
        assert!(is_full_width('漢'));
        assert!(is_full_width('、'));
        assert!(is_full_width('　'));
        assert!(!is_full_width('A'));
        assert!(!is_full_width('ｱ'));
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
        assert_eq!(
            normalize_whitespace("  Multiple   Spaces  "),
            "Multiple Spaces"
        );
    }

    #[test]
    fn test_half_width_katakana_to_full_width() {
        assert_eq!(half_width_katakana_to_full_width("ｶﾀｶﾅ"), "カタカナ");
        assert_eq!(half_width_katakana_to_full_width("ｶﾞｷﾞｸﾞｹﾞｺﾞ"), "ガギグゲゴ");
        assert_eq!(half_width_katakana_to_full_width("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ"), "パピプペポ");
        assert_eq!(half_width_katakana_to_full_width("ｳﾞﾜﾞｲﾞｴﾞｦﾞ"), "ヴヷヸヹヺ");
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
        assert_eq!(expand_iteration_marks("うゞ"), "うゔ");
        assert_eq!(expand_iteration_marks("いろゝゝ"), "いろろろ");
        assert_eq!(expand_iteration_marks("カヽヽ"), "カカカ");
        assert_eq!(expand_iteration_marks("トヽキ"), "トトキ");
        assert_eq!(expand_iteration_marks("カヾ"), "カガ");
        assert_eq!(expand_iteration_marks("ウヾ"), "ウヴ");
    }

    #[test]
    fn test_full_width_katakana_to_half_width() {
        assert_eq!(full_width_katakana_to_half_width("カタカナ"), "ｶﾀｶﾅ");
        assert_eq!(full_width_katakana_to_half_width("ガギグ"), "ｶﾞｷﾞｸﾞ");
        assert_eq!(full_width_katakana_to_half_width("パピプ"), "ﾊﾟﾋﾟﾌﾟ");
        assert_eq!(full_width_katakana_to_half_width("ヷヸヹヺ"), "ﾜﾞｲﾞｴﾞｦﾞ");
        assert_eq!(full_width_katakana_to_half_width("日本語ABC"), "日本語ABC");
    }

    #[test]
    fn test_dakuten_normalization() {
        assert_eq!(combine_dakuten("か\u{3099}ハ\u{309A}"), "がパ");
        assert_eq!(decompose_dakuten("がパ"), "か\u{3099}ハ\u{309A}");
        assert_eq!(combine_dakuten("e\u{301} か\u{3099}"), "e\u{301} が");
        assert_eq!(decompose_dakuten("é がパ"), "é か\u{3099}ハ\u{309A}");
        assert_eq!(
            combine_dakuten("ワ\u{3099}ヰ\u{3099}ヱ\u{3099}ヲ\u{3099}"),
            "ヷヸヹヺ"
        );
        assert_eq!(
            decompose_dakuten("ヷヸヹヺ"),
            "ワ\u{3099}ヰ\u{3099}ヱ\u{3099}ヲ\u{3099}"
        );
    }

    #[test]
    fn test_unicode_normalization() {
        assert_eq!(normalize_nfkc("ＡＢＣ１２３ｶﾞ"), "ABC123ガ");
        assert_eq!(normalize_nfc("か\u{3099}"), "が");
        assert_eq!(normalize_nfd("が"), "か\u{3099}");
        assert_eq!(normalize_nfd("é が"), "e\u{301} か\u{3099}");

        let options = NormalizeOptions {
            unicode: Some(UnicodeNormalizationForm::Nfd),
            ..NormalizeOptions::default()
        };
        assert_eq!(normalize_with_options("é が", &options), "e\u{301} が");
    }

    #[test]
    fn test_normalize_punctuation_brackets_symbols() {
        assert_eq!(normalize_punctuation("A，B．C､D｡"), "A、B。C、D。");
        assert_eq!(normalize_brackets_and_quotes("(\"本文\")"), "（「本文」）");
        assert_eq!(
            normalize_brackets_and_quotes("“本文” ‘注’"),
            "「本文」 『注』"
        );
        assert_eq!(normalize_symbols("コ〜ヒ～ - − —"), "コーヒー - - -");
    }

    #[test]
    fn test_old_kanji_and_variation_selectors() {
        assert_eq!(old_kanji_to_new("舊字體の國語"), "旧字体の国語");
        assert_eq!(remove_variation_selectors("葛\u{E0100}"), "葛");
    }

    #[test]
    fn test_character_type_ratios_and_analysis() {
        let ratios = character_type_ratios("あア漢A");
        assert_eq!(ratios.hiragana, 0.25);
        assert_eq!(ratios.katakana, 0.25);
        assert_eq!(ratios.kanji, 0.25);
        assert_eq!(ratios.ascii, 0.25);

        assert!(is_mostly_japanese("日本語です", 0.8));
        assert!(is_mostly_japanese("スーパー", 1.0));
        assert!(!is_mostly_japanese("ABC123", 0.5));
        assert!(has_mixed_scripts("日本語ABC"));
        assert_eq!(extract_japanese("ABC日本語123"), "日本語");
        assert_eq!(extract_japanese("ABCスーパー123"), "スーパー");
        assert_eq!(extract_ascii("ABC日本語123"), "ABC123");
        assert_eq!(remove_symbols("日本語、ABC!"), "日本語ABC");
        assert_eq!(remove_symbols("スーパー、コーヒー!"), "スーパーコーヒー");
        assert_eq!(remove_symbols("日本語！＃【ABC】※"), "日本語ABC");
        assert_eq!(remove_symbols("日本語　ABC DEF！"), "日本語　ABC DEF");
    }

    #[test]
    fn test_normalize_default_and_options() {
        assert_eq!(normalize("ＡＢＣ　ｶﾞｷﾞｸﾞ，舊字體"), "ABC ガギグ、旧字体");
        assert_eq!(normalize("コ～ヒ～とラ〜メン"), "コーヒーとラーメン");

        let options = NormalizeOptions {
            hiragana: true,
            half_width_ascii: true,
            punctuation: true,
            whitespace: WhitespaceMode::Collapse,
            ..NormalizeOptions::default()
        };
        assert_eq!(
            normalize_with_options("ＡＢＣ　カタカナ．", &options),
            "ABC かたかな。"
        );

        let decompose_options = NormalizeOptions {
            decompose_dakuten: true,
            ..NormalizeOptions::default()
        };
        assert_eq!(
            normalize_with_options("ｶﾞ ﾊﾟ ヴ", &decompose_options),
            "カ\u{3099} ハ\u{309A} ウ\u{3099}"
        );
    }

    #[test]
    fn test_normalizer_builder() {
        let normalizer = Normalizer::new()
            .hiragana(true)
            .half_width_ascii(true)
            .whitespace(WhitespaceMode::Collapse);

        assert_eq!(normalizer.normalize("ＡＢＣ　カタカナ"), "ABC かたかな");
    }

    #[test]
    fn test_normalizer_builder_last_direction_wins() {
        assert_eq!(
            Normalizer::new()
                .full_width_ascii(true)
                .half_width_ascii(true)
                .normalize("ＡＢＣ ABC"),
            "ABC ABC"
        );
        assert_eq!(
            Normalizer::new()
                .half_width_ascii(true)
                .full_width_ascii(true)
                .normalize("ＡＢＣ ABC"),
            "ＡＢＣ ＡＢＣ"
        );
        assert_eq!(
            Normalizer::new()
                .katakana(true)
                .hiragana(true)
                .normalize("カタカナ ひらがな"),
            "かたかな ひらがな"
        );
        assert_eq!(
            Normalizer::new()
                .half_width_katakana(false)
                .full_width_katakana(true)
                .normalize("カタカナ ｶﾀｶﾅ"),
            "ｶﾀｶﾅ ｶﾀｶﾅ"
        );
    }

    #[test]
    fn test_normalizer_builder_controls_all_options() {
        let normalizer = Normalizer::new()
            .unicode(UnicodeNormalizationForm::Nfkc)
            .unicode_normalization(None)
            .half_width_ascii(false)
            .half_width_katakana(false)
            .combine_dakuten(false)
            .decompose_dakuten(true)
            .punctuation(false)
            .brackets(false)
            .symbols(false)
            .old_kanji(false)
            .remove_variation_selectors(false)
            .expand_iteration_marks(false)
            .preserve_ascii_tokens(true)
            .whitespace(WhitespaceMode::Preserve);

        assert_eq!(normalizer.options().unicode, None);
        assert!(normalizer.options().decompose_dakuten);
        assert!(!normalizer.options().combine_dakuten);
        assert_eq!(
            normalizer.normalize("舊字體，(カゝ) か\u{3099}"),
            "舊字體，(カゝ) か\u{3099}"
        );
    }

    #[test]
    fn test_preserve_ascii_tokens() {
        let options = NormalizeOptions {
            preserve_ascii_tokens: true,
            ..NormalizeOptions::default()
        };

        assert_eq!(
            normalize_with_options("URL https://example.com/a,b と ＡＢＣ，", &options),
            "URL https://example.com/a,b と ABC、"
        );
        assert_eq!(
            normalize_with_options(
                "参照 (https://example.com/a,b), mail: user.name@example.com.",
                &options
            ),
            "参照 （https://example.com/a,b）、 mail: user.name@example.com。"
        );
        assert_eq!(
            normalize_with_options("価格 1,234.50，版 1.2.3.", &options),
            "価格 1,234.50、版 1.2.3."
        );
        assert_eq!(
            normalize_with_options("URL:https://example.com/a,b.", &options),
            "URL:https://example.com/a,b。"
        );
        assert_eq!(
            normalize_with_options("mail:user.name@example.com.", &options),
            "mail:user.name@example.com。"
        );
    }

    proptest! {
        #[test]
        fn prop_full_half_ascii_roundtrip(input in "[ -~]*") {
            prop_assert_eq!(to_half_width(&to_full_width(&input)), input);
        }

        #[test]
        fn prop_kana_roundtrip(input in "[ぁ-ゖ]*") {
            prop_assert_eq!(to_hiragana(&to_katakana(&input)), input);
        }
    }
}

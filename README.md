# japanese-text

日本語テキスト正規化のための軽量なRustライブラリ

[![Crates.io](https://img.shields.io/crates/v/japanese-text.svg)](https://crates.io/crates/japanese-text)
[![Documentation](https://docs.rs/japanese-text/badge.svg)](https://docs.rs/japanese-text)
[![License](https://img.shields.io/crates/l/japanese-text.svg)](https://github.com/0809android/japanese-text#license)

## 特徴

- ✨ **全角⇔半角変換** - ASCII文字の相互変換
- ✨ **カタカナ⇔ひらがな変換** - 日本語文字の相互変換
- ✨ **半角カタカナ⇔全角カタカナ変換** - 濁点・半濁点も正しく処理
- ✨ **Unicode正規化** - NFC/NFD/NFKC/NFKDに対応
- ✨ **濁点・半濁点処理** - 結合・分解の両方に対応
- ✨ **句読点・括弧・記号正規化** - 表記ゆれを統一
- ✨ **旧字体→新字体変換** - 代表的な旧字体を新字体へ変換
- ✨ **異体字セレクタ除去** - 検索・比較向けに文字を正規化
- ✨ **文字種判定** - ひらがな、カタカナ、漢字、全角文字の判定
- ✨ **文字種カウント・比率計算** - 文字列内の各文字種を分析
- ✨ **空白正規化** - 全角スペース、タブなどを統一
- ✨ **長音記号正規化** - 〜、～をーに統一
- ✨ **繰り返し記号展開** - ゝ、ゞ、ヽ、ヾを展開
- ✨ **一括正規化API** - `normalize`、`NormalizeOptions`、`Normalizer`を提供
- ✨ **シンプルなAPI** - 使いやすい関数群
- ✨ **充実したテスト** - ユニットテストとドキュメントテストで主要機能を検証

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
japanese-text = "0.2.0"
```

## 基本的な使い方

```rust
use japanese_text::*;

fn main() {
    // 全角→半角変換
    let half = to_half_width("ＡＢＣ１２３");
    assert_eq!(half, "ABC123");

    // 半角→全角変換
    let full = to_full_width("ABC123");
    assert_eq!(full, "ＡＢＣ１２３");

    // カタカナ→ひらがな変換
    let hiragana = to_hiragana("カタカナ");
    assert_eq!(hiragana, "かたかな");

    // ひらがな→カタカナ変換
    let katakana = to_katakana("ひらがな");
    assert_eq!(katakana, "ヒラガナ");

    // 半角カタカナ→全角カタカナ変換
    let full_kana = half_width_katakana_to_full_width("ｶﾀｶﾅ");
    assert_eq!(full_kana, "カタカナ");

    // 一括正規化
    let normalized = normalize("ＡＢＣ　ｶﾞｷﾞｸﾞ，舊字體");
    assert_eq!(normalized, "ABC ガギグ、旧字体");

    // 文字種判定
    assert_eq!(is_hiragana('あ'), true);
    assert_eq!(is_katakana('ア'), true);
    assert_eq!(is_kanji('漢'), true);
}
```

## API リファレンス

### 全角 / 半角 変換

#### `to_half_width(input: &str) -> String`

全角ASCII文字を半角に変換します。

```rust
assert_eq!(to_half_width("ＡＢＣ"), "ABC");
assert_eq!(to_half_width("１２３"), "123");
assert_eq!(to_half_width("！＠＃"), "!@#");
```

#### `to_full_width(input: &str) -> String`

半角ASCII文字を全角に変換します。

```rust
assert_eq!(to_full_width("ABC"), "ＡＢＣ");
assert_eq!(to_full_width("123"), "１２３");
```

### カタカナ / ひらがな 変換

#### `to_hiragana(input: &str) -> String`

カタカナをひらがなに変換します。

```rust
assert_eq!(to_hiragana("カタカナ"), "かたかな");
assert_eq!(to_hiragana("ヷヸヹヺ"), "わ\u{3099}ゐ\u{3099}ゑ\u{3099}を\u{3099}");
```

#### `to_katakana(input: &str) -> String`

ひらがなをカタカナに変換します。

```rust
assert_eq!(to_katakana("ひらがな"), "ヒラガナ");
```

### 半角カタカナ変換

#### `half_width_katakana_to_full_width(input: &str) -> String`

半角カタカナを全角カタカナに変換します。濁点（゛）と半濁点（゜）も正しく結合されます。

```rust
assert_eq!(half_width_katakana_to_full_width("ｶﾀｶﾅ"), "カタカナ");
assert_eq!(half_width_katakana_to_full_width("ｶﾞｷﾞｸﾞ"), "ガギグ");
assert_eq!(half_width_katakana_to_full_width("ﾊﾟﾋﾟﾌﾟ"), "パピプ");
```

#### `full_width_katakana_to_half_width(input: &str) -> String`

全角カタカナを半角カタカナに変換します。

```rust
assert_eq!(full_width_katakana_to_half_width("カタカナ"), "ｶﾀｶﾅ");
assert_eq!(full_width_katakana_to_half_width("ガギグ"), "ｶﾞｷﾞｸﾞ");
assert_eq!(full_width_katakana_to_half_width("パピプ"), "ﾊﾟﾋﾟﾌﾟ");
```

### 文字種判定

#### `is_hiragana(c: char) -> bool`

文字がひらがなかどうかを判定します。

```rust
assert_eq!(is_hiragana('あ'), true);
assert_eq!(is_hiragana('ア'), false);
```

#### `is_katakana(c: char) -> bool`

文字がカタカナかどうかを判定します。

```rust
assert_eq!(is_katakana('ア'), true);
assert_eq!(is_katakana('ー'), true);
assert_eq!(is_katakana('ヷ'), true);
assert_eq!(is_katakana('あ'), false);
```

#### `is_half_width_katakana(c: char) -> bool`

文字が半角カタカナかどうかを判定します。

```rust
assert_eq!(is_half_width_katakana('ｱ'), true);
assert_eq!(is_half_width_katakana('｡'), false);
```

#### `is_kanji(c: char) -> bool`

文字が漢字（CJK統合漢字）かどうかを判定します。

```rust
assert_eq!(is_kanji('漢'), true);
assert_eq!(is_kanji('字'), true);
```

#### `is_full_width(c: char) -> bool`

文字が全角文字かどうかを判定します。

```rust
assert_eq!(is_full_width('Ａ'), true);
assert_eq!(is_full_width('１'), true);
assert_eq!(is_full_width('ア'), true);
assert_eq!(is_full_width('漢'), true);
assert_eq!(is_full_width('A'), false);
```

### 文字種カウント

#### `count_character_types(input: &str) -> CharacterTypes`

文字列内の各文字種の数をカウントします。

```rust
let counts = count_character_types("あア漢ABC123");
println!("ひらがな: {}", counts.hiragana);  // 1
println!("カタカナ: {}", counts.katakana);  // 1
println!("漢字: {}", counts.kanji);        // 1
println!("ASCII: {}", counts.ascii);        // 6
```

#### `character_type_ratios(input: &str) -> CharacterTypeRatios`

文字種ごとの比率を計算します。

```rust
let ratios = character_type_ratios("あア漢A");
assert_eq!(ratios.hiragana, 0.25);
assert_eq!(ratios.katakana, 0.25);
assert_eq!(ratios.kanji, 0.25);
assert_eq!(ratios.ascii, 0.25);
```

#### 分析・抽出

```rust
assert!(is_mostly_japanese("日本語です", 0.8));
assert!(is_mostly_japanese("スーパー", 1.0));
assert!(has_mixed_scripts("日本語ABC"));
assert_eq!(extract_japanese("ABC日本語123"), "日本語");
assert_eq!(extract_japanese("ABCスーパー123"), "スーパー");
assert_eq!(extract_ascii("ABC日本語123"), "ABC123");
assert_eq!(remove_symbols("日本語、ABC!"), "日本語ABC");
```

### テキスト正規化

#### `normalize_whitespace(input: &str) -> String`

文字列内の空白文字を正規化します（全角スペース、タブなどを半角スペースに統一）。

```rust
assert_eq!(normalize_whitespace("Hello　World"), "Hello World");
assert_eq!(normalize_whitespace("A\t\tB"), "A B");
```

#### `normalize_prolonged_sound(input: &str) -> String`

長音記号を正規化します（〜、～をーに統一）。

```rust
assert_eq!(normalize_prolonged_sound("コ〜ヒ〜"), "コーヒー");
```

#### `expand_iteration_marks(input: &str) -> String`

繰り返し記号を展開します。

```rust
assert_eq!(expand_iteration_marks("いろゝ"), "いろろ");
assert_eq!(expand_iteration_marks("かゞ"), "かが");
```

#### Unicode・濁点正規化

```rust
assert_eq!(normalize_nfkc("ＡＢＣ１２３ｶﾞ"), "ABC123ガ");
assert_eq!(combine_dakuten("か\u{3099}"), "が");
assert_eq!(decompose_dakuten("パ"), "ハ\u{309A}");
```

#### 句読点・記号・旧字体・異体字セレクタ

```rust
assert_eq!(normalize_punctuation("A，B．C､D｡"), "A、B。C、D。");
assert_eq!(normalize_brackets_and_quotes("(\"本文\")"), "（「本文」）");
assert_eq!(normalize_symbols("コ〜ヒ～ −"), "コーヒー -");
assert_eq!(old_kanji_to_new("舊字體の國語"), "旧字体の国語");
assert_eq!(remove_variation_selectors("葛\u{E0100}"), "葛");
```

#### 一括正規化

```rust
assert_eq!(normalize("ＡＢＣ　ｶﾞｷﾞｸﾞ，舊字體"), "ABC ガギグ、旧字体");

let normalizer = Normalizer::new()
    .hiragana(true)
    .half_width_ascii(true)
    .whitespace(WhitespaceMode::Collapse);

assert_eq!(normalizer.normalize("ＡＢＣ　カタカナ"), "ABC かたかな");
```

## ユースケース

- ユーザー入力の正規化
- 検索用のテキスト前処理
- データクリーニング
- レガシーシステムとの連携（半角カタカナ変換）
- 表示用のフォーマット変換
- 日本語テキスト分析

## サンプル実行

```bash
# サンプルプログラムを実行
cargo run --example basic

# テストを実行
cargo test

# ドキュメントを生成
cargo doc --open
```

## パフォーマンス

このライブラリは、よく使う文字変換はシンプルな文字マッピングで処理し、Unicode正規化は実績のある`unicode-normalization`に委ねます。用途に応じて個別関数と一括正規化APIを使い分けられます。

## コントリビューション

プルリクエストを歓迎します！気軽にご投稿ください。

## ライセンス

このプロジェクトは、以下のいずれかのライセンスでデュアルライセンスされています：

- MITライセンス ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)

お好みのライセンスをお選びください。

## 謝辞

このライブラリは、大規模な日本語テキスト処理ライブラリの軽量な代替として、最も一般的なテキスト正規化のニーズに焦点を当てて作成されました。

---

## English Summary

A lightweight Rust library for Japanese text normalization, supporting:
- Full-width ⇔ Half-width conversion for ASCII characters
- Katakana ⇔ Hiragana conversion
- Half-width ⇔ Full-width Katakana conversion (with dakuten/handakuten support)
- Unicode normalization
- Punctuation, bracket, symbol, old kanji, and variation selector normalization
- Character type detection (hiragana, katakana, kanji, etc.)
- Character type counting and ratios
- Builder-style batch normalization
- Whitespace normalization
- Prolonged sound mark normalization
- Iteration mark expansion
- Unicode normalization powered by `unicode-normalization`

# japanese-text

日本語テキスト正規化のための軽量なRustライブラリ

[![Crates.io](https://img.shields.io/crates/v/japanese-text.svg)](https://crates.io/crates/japanese-text)
[![Documentation](https://docs.rs/japanese-text/badge.svg)](https://docs.rs/japanese-text)
[![License](https://img.shields.io/crates/l/japanese-text.svg)](https://github.com/0809android/japanese-text#license)

## 特徴

- ✨ **全角⇔半角変換** - ASCII文字の相互変換
- ✨ **カタカナ⇔ひらがな変換** - 日本語文字の相互変換
- ✨ **半角カタカナ→全角カタカナ変換** - 濁点・半濁点も正しく処理
- ✨ **文字種判定** - ひらがな、カタカナ、漢字、全角文字の判定
- ✨ **文字種カウント** - 文字列内の各文字種をカウント
- ✨ **空白正規化** - 全角スペース、タブなどを統一
- ✨ **長音記号正規化** - 〜、～をーに統一
- ✨ **繰り返し記号展開** - ゝ、ゞ、ヽ、ヾを展開
- ✨ **ゼロ依存** - 純粋なRust実装、外部依存なし
- ✨ **シンプルなAPI** - 使いやすい関数群
- ✨ **充実したテスト** - 包括的なテストカバレッジ（32個のテスト）

## インストール

`Cargo.toml`に以下を追加してください：

```toml
[dependencies]
japanese-text = "0.1.0"
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
assert_eq!(is_katakana('あ'), false);
```

#### `is_half_width_katakana(c: char) -> bool`

文字が半角カタカナかどうかを判定します。

```rust
assert_eq!(is_half_width_katakana('ｱ'), true);
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

このライブラリは、外部依存や複雑なロジックなしにシンプルな文字マッピングを使用しているため、非常に高速でオーバーヘッドが最小限です。

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
- Half-width Katakana → Full-width Katakana conversion (with dakuten/handakuten support)
- Character type detection (hiragana, katakana, kanji, etc.)
- Character type counting
- Whitespace normalization
- Prolonged sound mark normalization
- Iteration mark expansion
- Zero dependencies, pure Rust implementation

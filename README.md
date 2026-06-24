# js-string-escape

[![crates.io](https://img.shields.io/crates/v/js-string-escape.svg)](https://crates.io/crates/js-string-escape)
[![docs.rs](https://docs.rs/js-string-escape/badge.svg)](https://docs.rs/js-string-escape)
[![CI](https://github.com/trananhtung/js-string-escape/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/js-string-escape/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/js-string-escape.svg)](#license)

**Escape a string for use as a JavaScript string literal body.**

Escapes the characters that aren't valid inside a single- or double-quoted JavaScript string
literal: `"`, `'`, `\`, and the line terminators `\n`, `\r`, `U+2028`, and `U+2029`.
Everything else passes through unchanged. A faithful Rust port of the
[`js-string-escape`](https://www.npmjs.com/package/js-string-escape) npm package.

- **Zero dependencies**, **`#![no_std]`**
- Differential-tested against the reference `js-string-escape` implementation

## Install

```toml
[dependencies]
js-string-escape = "0.1"
```

## Usage

```rust
use js_string_escape::js_string_escape;

assert_eq!(js_string_escape(r#"a"b'c\d"#), r#"a\"b\'c\\d"#);
assert_eq!(js_string_escape("line1\nline2"), "line1\\nline2");

// Tabs, other control characters, and non-ASCII pass through:
assert_eq!(js_string_escape("café\t😀"), "café\t😀");
```

For richer escaping (ASCII-only output, JSON mode, backtick quotes, …) see
[`jsesc`](https://crates.io/crates/jsesc).

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.

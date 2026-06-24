//! # js-string-escape — escape a string for a JavaScript string literal
//!
//! Escape the characters that aren't valid inside a single- or double-quoted JavaScript
//! string literal — `"`, `'`, `\`, and the line terminators `\n`, `\r`, `U+2028`, and
//! `U+2029`. Everything else (including tabs, other control characters, and non-ASCII) is
//! left as-is. A faithful Rust port of the
//! [`js-string-escape`](https://www.npmjs.com/package/js-string-escape) npm package.
//!
//! ```
//! use js_string_escape::js_string_escape;
//!
//! assert_eq!(js_string_escape(r#"a"b'c\d"#), r#"a\"b\'c\\d"#);
//! assert_eq!(js_string_escape("line1\nline2"), "line1\\nline2");
//! assert_eq!(js_string_escape("café\t"), "café\t"); // non-ASCII and tab pass through
//! ```
//!
//! This is the minimal escaping needed to embed arbitrary text into a JS string literal; for
//! richer options (ASCII-only output, JSON mode, …) see
//! [`jsesc`](https://crates.io/crates/jsesc).
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/js-string-escape/0.1.0")]

extern crate alloc;

use alloc::string::String;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Escape `string` so it can be embedded in a JavaScript string literal.
///
/// Only `"`, `'`, `\`, `\n`, `\r`, `U+2028`, and `U+2029` are escaped; all other characters
/// pass through unchanged.
///
/// ```
/// # use js_string_escape::js_string_escape;
/// assert_eq!(js_string_escape("say \"hi\""), "say \\\"hi\\\"");
/// ```
#[must_use]
pub fn js_string_escape(string: &str) -> String {
    let mut out = String::with_capacity(string.len());
    for character in string.chars() {
        match character {
            '"' | '\'' | '\\' => {
                out.push('\\');
                out.push(character);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            _ => out.push(character),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quotes_and_backslash() {
        assert_eq!(js_string_escape("a\"b"), "a\\\"b");
        assert_eq!(js_string_escape("a'b"), "a\\'b");
        assert_eq!(js_string_escape("a\\b"), "a\\\\b");
    }

    #[test]
    fn escapes_line_terminators() {
        assert_eq!(js_string_escape("a\nb"), "a\\nb");
        assert_eq!(js_string_escape("a\rb"), "a\\rb");
        assert_eq!(js_string_escape("a\u{2028}b"), "a\\u2028b");
        assert_eq!(js_string_escape("a\u{2029}b"), "a\\u2029b");
    }

    #[test]
    fn leaves_other_characters_untouched() {
        // Tab, backspace, form feed, NUL, backtick, and non-ASCII all pass through.
        assert_eq!(js_string_escape("a\tb"), "a\tb");
        assert_eq!(js_string_escape("a\u{0}b"), "a\u{0}b");
        assert_eq!(js_string_escape("x`y"), "x`y");
        assert_eq!(js_string_escape("café 😀"), "café 😀");
        assert_eq!(js_string_escape(""), "");
        assert_eq!(js_string_escape("plain"), "plain");
    }
}

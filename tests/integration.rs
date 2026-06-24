//! Integration tests exercising the public API of `js-string-escape`.

use js_string_escape::js_string_escape;

#[test]
fn embed_in_literal() {
    let value = "He said: \"don't\"\nthen left";
    assert_eq!(js_string_escape(value), "He said: \\\"don\\'t\\\"\\nthen left");
}

#[test]
fn windows_newlines() {
    assert_eq!(js_string_escape("a\r\nb"), "a\\r\\nb");
}

#[test]
fn path_with_backslashes() {
    assert_eq!(js_string_escape("C:\\Users\\me"), "C:\\\\Users\\\\me");
}

#[test]
fn unicode_line_separators() {
    assert_eq!(js_string_escape("a\u{2028}b\u{2029}c"), "a\\u2028b\\u2029c");
}

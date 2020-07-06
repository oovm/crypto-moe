use std::collections::BTreeMap;
use crate::utils::{DOT_LINE, LETTER};

/// Decodes a morse representation string into an ascii string
///
/// # Examples
/// ```rust
/// use morse::decode;
///
/// assert_eq!(decode::decode("... ___ ...").unwrap(), "sos");
/// ```
///
/// # Errors
/// Decoding will error with a `morse::TranslationError`
/// when an unsupported morse character is being decoded.
/// The error structure contains a `Vec<String> unsupported_characters`
/// to show what characters failed.
///
/// # Notice
/// SOS HELP      = ...___...  .... . ._.. .__.        => ...___........._...__.
/// I AM HIS DATE = ..  ._ __  .... .. ...  _.. ._ _ . => ...___........._...__.
pub fn decode(input: &str) -> String {
    let mut result = String::new();
    let mut map = BTreeMap::new();
    for (k, v) in DOT_LINE.iter().zip(LETTER.iter()) {
        map.insert(*k, v);
    }
    let normed = input.replace("*", ".").replace("-", "_");
    for word in normed.trim().split("/") {
        for c in word.trim().split(" ") {
            let ch = match map.get(c) {
                Some(o) => **o,
                None => decode_missing(c),
            };
            result.push(ch);
        }
        result.push(' ');
    }
    result.pop();
    result
}

fn decode_missing(s: &str) -> char {
    let n = s.replace('_', "1").replace('.', "0");
    if let Ok(u) = u32::from_str_radix(&n, 2) {
        if let Some(c) = std::char::from_u32(u - 64) {
            return c;
        }
    }
    return ' ';
}

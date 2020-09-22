use crate::utils::{DOT_LINE, LETTER};
use std::collections::BTreeMap;

/// Encodes an ascii string into a morse code representation
///
/// # Examples
/// ```
/// use crypto_morse::encode;
///
/// assert_eq!(encode("SOS"), "... ___ ...");
/// ```
pub fn encode(input: &str) -> String {
    encode_raw(input.to_lowercase().trim())
}

/// Encodes an ascii string into a morse code representation
///
/// # Examples
/// ```
/// use crypto_morse::encode_raw;
///
/// assert_eq!(encode_raw("SOS"), "... ___ ...");
/// ```
pub fn encode_raw(input: &str) -> String {
    let mut result = String::with_capacity(10 * input.len());
    let mut map = BTreeMap::new();
    for (k, v) in LETTER.iter().zip(DOT_LINE.iter()) {
        map.insert(*k, v);
    }
    for c in input.chars() {
        // if c.is_whitespace() {result.pop();result.push('/');continue;}
        let s = match map.get(&c) {
            Some(o) => String::from(**o),
            None => encode_missing(c),
        };
        result.push_str(&s);
        result.push(' ');
    }
    result.pop();
    return result;
}

fn encode_missing(c: char) -> String {
    format!("{:b}", c as u32 + 64).replace('1', "_").replace('0', ".")
}

//! doc utils

use lazy_static::lazy_static;
use std::collections::BTreeMap;

const LATIN_LOWER0: &str = "abcdefghijklmnopqrstuvwxyz";
const LATIN_LOWER1: &str = "ɐqɔpǝɟᵷɥᴉfʞꞁɯuodbɹsʇnʌʍxʎz";

const LATIN_UPPER0: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LATIN_UPPER1: &str = "ⱯBƆDƎℲ⅁HIſꞰꞀƜNOԀÒᴚSꞱ∩ɅʍX⅄Z";

const DIGIT0: &str = "0123456789";
const DIGIT1: &str = "012Ɛᔭ59Ɫ86";

const PUNCTUATION0: &str = ",.;!?&†()[]{}";
const PUNCTUATION1: &str = "⸲˙⸵¡¿⅋⸸)(][}{";

lazy_static! {
/// const STRING_NORMAL: &str = concat!(LATIN_LOWER0, LATIN_UPPER0, DIGIT0, PUNCTUATION0);
    pub static ref MAP_NORMAL: BTreeMap<char, char> = {
        let normal: String = vec![LATIN_LOWER0, LATIN_UPPER0, DIGIT0, PUNCTUATION0].join("");
        let invert: String = vec![LATIN_LOWER1, LATIN_UPPER1, DIGIT1, PUNCTUATION1].join("");
        let mut map = BTreeMap::default();
        for (i, j) in normal.chars().zip(invert.chars()) {
            map.insert(i, j);
        }
        return map;
    };
    /// const STRING_INVERT: &str = concat!(LATIN_LOWER1, LATIN_UPPER1, DIGIT1, PUNCTUATION1);
    pub static ref MAP_INVERT: BTreeMap<char, char> = {
        let normal: String = vec![LATIN_LOWER0, LATIN_UPPER0, DIGIT0, PUNCTUATION0].join("");
        let invert: String = vec![LATIN_LOWER1, LATIN_UPPER1, DIGIT1, PUNCTUATION1].join("");
        let mut map = BTreeMap::default();
        for (i, j) in invert.chars().zip(normal.chars()) {
            map.insert(i, j);
        }
        return map;
    };
}

/// Encodes an ascii string into the invert representation
///
/// # Examples
/// ```rust
/// use crypto_invert::encode;
///
/// let r1 = encode("i love you!");
/// let r2 = encode("I LOVE YOU!");
/// assert_eq!(r1, "ᴉ ꞁoʌǝ ʎon¡");
/// assert_eq!(r2, "I ꞀOɅƎ ⅄O∩¡");
/// ```
pub fn encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match MAP_NORMAL.get(&c) {
            None => out.push(c),
            Some(s) => out.push(*s),
        }
    }
    return out;
}

/// Decoding the invert representation to normal string
///
/// # Examples
/// ```rust
/// use crypto_invert::{decode, encode};
///
/// let r1 = "i love you!";
/// let r2 = "I LOVE YOU!";
/// assert_eq!(decode(&encode(r1)), r1);
/// assert_eq!(decode(&encode(r2)), r2);
/// ```
pub fn decode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match MAP_INVERT.get(&c) {
            None => out.push(c),
            Some(s) => out.push(*s),
        }
    }
    return out;
}

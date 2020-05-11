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

// const STRING_NORMAL: &str = concat!(LATIN_LOWER0, LATIN_UPPER0, DIGIT0, PUNCTUATION0);
// const STRING_INVERT: &str = concat!(LATIN_LOWER1, LATIN_UPPER1, DIGIT1, PUNCTUATION1);

lazy_static! {
    pub static ref MAP_NORMAL: BTreeMap<char, char> = {
        let normal: String = vec![LATIN_LOWER0, LATIN_UPPER0, DIGIT0, PUNCTUATION0].join("");
        let invert: String = vec![LATIN_LOWER1, LATIN_UPPER1, DIGIT1, PUNCTUATION1].join("");
        let mut map = BTreeMap::default();
        for (i, j) in normal.chars().zip(invert.chars()) {
            map.insert(i, j);
        }
        return map;
    };
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

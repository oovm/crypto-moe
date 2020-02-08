use crate::auxiliary::CHAR_SET;
use crypto_random_map::SecretDense;
use encoding_rs::GB18030;
use flate2::{write::DeflateEncoder, Compression};
use rand::Rng;
use std::io::Write;

fn cycle_xor(vec: &mut Vec<u8>) -> Vec<u8> {
    let s = rand::thread_rng().gen::<u8>();
    for i in vec.iter_mut() {
        *i ^= s;
    }
    vec.reverse();
    vec.push(s);
    vec.to_owned()
}

fn compress(input: &str) -> Vec<u8> {
    let (cow, _encoding_used, _had_errors) = GB18030.encode(input);
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&cow[..]).unwrap();
    let mut compressed = encoder.finish().unwrap();
    cycle_xor(&mut compressed);
    cycle_xor(&mut compressed);
    return compressed;
}

fn insert_dot(mapped: Vec<char>) -> Vec<char> {
    if mapped.len() <= 6 {
        return mapped;
    }
    let mut result: Vec<char> = vec![];
    let mut r = rand::thread_rng().gen_range(2, 5 + 1);
    for i in mapped {
        if r > 0 {
            result.push(i);
            r -= 1
        }
        else {
            result.push('·');
            r = rand::thread_rng().gen_range(1, 8);
            result.push(i);
        }
    }
    return result;
}

pub fn encode(s: &str) -> String {
    let compressed = compress(s);
    let sec = SecretDense::new(CHAR_SET);
    let mapped = sec.encode(&compressed);
    insert_dot(mapped).iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_debug(s: &str) -> String {
        println!("Raw size: {}", s.len());
        let compressed = compress(s);
        println!("Compressed: {}", compressed.len());
        let sec = SecretDense::new(CHAR_SET);
        let mapped = sec.encode(&compressed);
        println!("Transformed: {}", mapped.len());
        println!();
        insert_dot(mapped).iter().collect()
    }

    #[test]
    fn short() {
        let input = "0";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        debug_assert_ne!(r1, r2)
    }

    #[test]
    fn middle() {
        let input = "苟利国家生死以, 岂因祸福避趋之?";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        debug_assert_ne!(r1, r2)
    }
}

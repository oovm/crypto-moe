use crate::auxiliary::{div_rem, CHAR_SET};
use encoding_rs::GB18030;
use flate2::write::DeflateEncoder;
use flate2::Compression;
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

fn cycle_and(vec: &mut Vec<u8>) -> Vec<u8> {
    let s = rand::thread_rng().gen::<u8>();
    for i in vec.iter_mut() {
        *i &= s;
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
    cycle_and(&mut compressed);
    cycle_xor(&mut compressed);
    return compressed;
}

fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x, y)
}

fn transform(v: Vec<u8>, radix: usize) -> Vec<usize> {
    let mut s: Vec<usize> = vec![];
    let mut q: (usize, usize);
    let mut r: usize = 0;
    for i in v.iter() {
        r = r * 256 + *i as usize;
        if r >= radix {
            // (q, r) = div_rem_usize(r, radix);
            q = div_rem_usize(r, radix);
            r = q.1;
            s.push(q.0)
        }
    }
    s.push(r);
    return s;
}

fn insert_dot(mapped: Vec<char>) -> Vec<char> {
    if mapped.len() <= 6 {
        return mapped;
    }
    let mut result: Vec<char> = vec![];
    let mut r = rand::thread_rng().gen_range(2, 6);
    for i in mapped {
        if r > 0 {
            result.push(i);
            r -= 1
        } else {
            result.push('·');
            r = rand::thread_rng().gen_range(1, 8);
            result.push(i);
        }
    }
    return result;
}

fn char_map(index: Vec<usize>) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in index.iter() {
        let c = CHAR_SET.chars().nth(*i).unwrap();
        result.push(c)
    }
    return result;
}

pub fn encode(s: &str) -> String {
    let compressed = compress(s);
    let index = transform(compressed, CHAR_SET.chars().count());
    let mapped = char_map(index);
    insert_dot(mapped).iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_debug(s: &str) -> String {
        println!("Raw size: {}", s.len());
        let compressed = compress(s);
        println!("Compressed: {}", compressed.len());
        println!("        {:?}", compressed);
        let index = transform(compressed, CHAR_SET.chars().count());
        println!("Transformed: {}", index.len());
        println!("        {:?}", index);
        let mapped = char_map(index);
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
        debug_assert_eq!(r1, r2)
    }
}

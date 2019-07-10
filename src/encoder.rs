use crate::auxiliary::CHAR_SET;
use convert_base::Convert;
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
        } else {
            result.push('·');
            r = rand::thread_rng().gen_range(1, 8);
            result.push(i);
        }
    }
    return result;
}

//Todo: Too slow
fn char_map(index: Vec<u8>) -> Vec<char> {
    let mut base = Convert::new(256, CHAR_SET.chars().count() as u64);
    let output = base.convert::<u8, u16>(&index);
    let mut result: Vec<char> = vec![];
    for i in output.iter() {
        let c = CHAR_SET.chars().nth(*i as usize).unwrap();
        result.push(c)
    }
    return result;
}

pub fn encode(s: &str) -> String {
    let compressed = compress(s);
    let mapped = char_map(compressed);
    insert_dot(mapped).iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_debug(s: &str) -> String {
        println!("Raw size: {}", s.len());
        let compressed = compress(s);
        println!("Compressed: {}", compressed.len());
        let mapped = char_map(compressed);
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
        let input = "薇桂·姣千·玲倩燢·琼爱英曦莉馥琴·乐丝琬淑·婉利·茹萍·纯滢毓兰裳寒娜丹·欢翼馥莺芊晗·红伤";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        debug_assert_eq!(r1, r2)
    }
}
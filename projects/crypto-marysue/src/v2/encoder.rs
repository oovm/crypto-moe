use crate::{
    auxiliary::ALIGNED,
    v1::encoder::{compress, insert_dot},
};

pub fn encode(s: &str) -> String {
    let compressed = compress(s);
    let mapped = ALIGNED.encode(&compressed);
    insert_dot(mapped).iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_debug(s: &str) -> String {
        println!("Raw size: {}", s.len());
        let compressed = compress(s);
        println!("Compressed: {}", compressed.len());
        let mapped = ALIGNED.encode(&compressed);
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

use crypto_aaencode::{decode, encode};

#[rustfmt::skip]
pub const TEST_195: &str = "\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
i love you 3000 times!i love you 3000 times!i love you 3000 times!\
";


fn test_encoding() {
    let r1 = encode(TEST_195);
    let r2 = encode(TEST_195);
    debug_assert_eq!(r1, r2)
}


#[rustfmt::skip]
fn test_decoding() {
    let secret = "莺血樱·安倾·沫芝娅澜倩·黎盘如娥凝纨文真·英澜倾倾离·妍墨丹利血枫澪·黎格夏魑璧铃莺·夏";
    let raw = "力微任重久神疲, 再竭衰庸定不支.";
    debug_assert_eq!(decode(secret), raw)
}


fn test_all() {
    let secret = "力微任重久神疲, 再竭衰庸定不支.";
    let result = decode(&encode(secret));
    assert_eq!(secret, result)
}

use crypto_marysue::{decode, encode};

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

#[test]
fn test_encoding() {
    let r1 = encode(TEST_195);
    let r2 = encode(TEST_195);
    assert_ne!(r1, r2)
}

#[test]
#[rustfmt::skip]
fn test_decoding() {
    let secret = "樱梦希樱·育琰珍蕊锦珍菲·苏珠莲莲燕迷·蝶珠莹·铃玲莉雨玲静·萦幻影珍邪蝶·玲芝苑燢茜·落珍·莳莺珍苏阳·珠莺蝶珊韵芸·巧如";
    let raw = "力微任重久神疲, 再竭衰庸定不支.";
    assert_eq!(decode(secret), raw)
}

#[test]
fn test_all() {
    let secret = "力微任重久神疲, 再竭衰庸定不支.";
    let result = decode(&encode(secret));
    assert_eq!(secret, result)
}

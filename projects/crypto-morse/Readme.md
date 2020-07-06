Unicode Morse Encoding
======================

```rust
extern crate crypto_morse;
use crypto_morse::{decode, encode, encode_raw};

#[test]
fn test_encoding() {
    assert_eq!(encode("SOS"), encode("sos"));
    assert_ne!(encode_raw("SOS"), encode("sos"));
}

#[test]
fn encode_word() {
    assert_eq!(encode("az"), "._ __..");
    assert_eq!(encode("AZ"), "._ __..");
    assert_eq!(encode_raw("AZ"), "_......_ _..__._.");
}

#[test]
fn encode_multiple_words() {
    assert_eq!(encode("中文"), "_..___..__.__._ __.._.___...___");
    assert_eq!(encode("abc xyz"), "._ _... _._. / _.._ _.__ __..");
    assert_eq!(encode_raw("A Z"), "_......_ / _..__._.");
}

#[test]
fn decode_word() {
    assert_eq!(decode("._ __.."), "az");
    assert_eq!(decode("_......_ _..__._."), "AZ");
}

#[test]
fn decode_multiple_words() {
    assert_eq!(decode("_..___..__.__._ __.._.___...___"), "中文");
    assert_eq!(decode("_......_ / _..__._."), "A Z");
}
```
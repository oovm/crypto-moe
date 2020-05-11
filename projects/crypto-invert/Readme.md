Unicode Invert Text
===================

```rust
extern crate crypto_invert;
use crypto_invert::{decode, encode};

#[test]
fn test_encoding() {
    let r1 = encode("i love you!");
    let r2 = encode("I LOVE YOU!");
    assert_eq!(r1, "ᴉ ꞁoʌǝ ʎon¡");
    assert_eq!(r2, "I ꞀOɅƎ ⅄O∩¡");
}

#[test]
fn test_decoding() {
    let r1 = "i love you!";
    let r2 = "I LOVE YOU!";
    assert_eq!(decode(&encode(r1)), r1);
    assert_eq!(decode(&encode(r2)), r2);
}
```

There are thousands of possibilities in ciphertext, but there is only one corresponding information.

Notice that the version number used for encryption and decryption **must be the same**, because the key will keep expanding.
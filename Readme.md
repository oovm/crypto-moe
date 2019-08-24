Marysue™ Encoding
=================

UTF8 -> GB18030 -> Xor -> Xor -> BaseX

```rust
extern crate crypto_marysue;
use crypto_marysue::{decode, encode};

#[test]
fn test_encoding() {
    let input = "力微任重久神疲, 再竭衰庸定不支.";
    let r1 = encode(input);
    let r2 = encode(input);
    debug_assert_ne!(r1, r2)
}

#[test]
fn test_decoding() {
    let secret = "晶凌娅萦弥·琉婷·清梅凝琴妙阳嫩音·淑颖宁凌·淑寇盘陌菁城·烟仪贞纱翠·佳素寂洛姬贞·碎墨";
    let raw = "力微任重久神疲, 再竭衰庸定不支.";
    debug_assert_eq!(decode(secret), raw)
}
```
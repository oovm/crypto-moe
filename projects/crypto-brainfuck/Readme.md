Brainfuck™ Encoding
===================

```rust
use crypto_brainfuck::encode;

#[test]
fn test() {
    let out = encode("Hello, World!");
    let target = "-[++[<++>->+++>+++<<]---->+]<<<<.<<<<-.<..<<+.<<<<.>>.>>>-.<.+++.>>.>-.<<<<<+.";
    assert_eq!(out.replace("\n",""), target)
}
```

If there are 14 consecutive `+` or `-` signs, then this is a bad case of the algorithm.

See: [Brainfuck constants](https://esolangs.org/wiki/Brainfuck_constants)

Some other: https://copy.sh/brainfuck/text.html
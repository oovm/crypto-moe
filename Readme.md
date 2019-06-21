Marysue™ Encoding
=================

UTF8 -> GB18030 -> Xor -> Xor -> BaseX

```rust
    use super::*;

    #[test]
    fn encode() {
        let input = "薇桂·姣千·玲倩燢·琼爱英曦莉馥琴·乐丝琬淑·婉利·茹萍·纯滢毓兰裳寒娜丹·欢翼馥莺芊晗·红伤";
        let r1 = encode_debug(input);
        let r2 = encode_debug(input);
        debug_assert_eq!(r1, r2)
    }
```
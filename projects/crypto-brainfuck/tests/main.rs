use crypto_brainfuck::encode;

#[test]
fn test() {
    let out = encode("Hello, World!");
    let target = "-[++[<++>->+++>+++<<]---->+]<<<<.<<<<-.<..<<+.<<<<.>>.>>>-.<.+++.>>.>-.<<<<<+.";
    assert_eq!(out.replace("\n", ""), target)
}

#[test]
fn test_short() {
    let out = encode("A");
    let target = "+[+[<]>>+<+]>.";
    assert_eq!(out.replace("\n", ""), target)
}

#[test]
fn test_short2() {
    let out = encode("a");
    let target = "+[-[---<]>>-]<-.";
    assert_eq!(out.replace("\n", ""), target)
}

#[test]
fn test_short3() {
    let out = encode("0");
    let target = "-[>+<-----]>---.";
    assert_eq!(out.replace("\n", ""), target)
}

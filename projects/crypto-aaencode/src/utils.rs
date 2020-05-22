const HEAD: [&str; 21] = [
    "ﾟωﾟﾉ= /｀ｍ´）ﾉ ~┻━┻   //*´∇｀*/ ['_']; o=(ﾟｰﾟ)  =_=3; c=(ﾟΘﾟ) =(ﾟｰﾟ)-(ﾟｰﾟ); ",
    "(ﾟДﾟ) =(ﾟΘﾟ)= (o^_^o)/ (o^_^o);",
    "(ﾟДﾟ)={ﾟΘﾟ: '_' ,ﾟωﾟﾉ : ((ﾟωﾟﾉ==3) +'_') [ﾟΘﾟ] ",
    ",ﾟｰﾟﾉ :(ﾟωﾟﾉ+ '_')[o^_^o -(ﾟΘﾟ)] ",
    ",ﾟДﾟﾉ:((ﾟｰﾟ==3) +'_')[ﾟｰﾟ] }; (ﾟДﾟ) [ﾟΘﾟ] =((ﾟωﾟﾉ==3) +'_') [c^_^o];",
    "(ﾟДﾟ) ['c'] = ((ﾟДﾟ)+'_') [ (ﾟｰﾟ)+(ﾟｰﾟ)-(ﾟΘﾟ) ];",
    "(ﾟДﾟ) ['o'] = ((ﾟДﾟ)+'_') [ﾟΘﾟ];",
    "(ﾟoﾟ)=(ﾟДﾟ) ['c']+(ﾟДﾟ) ['o']+(ﾟωﾟﾉ +'_')[ﾟΘﾟ]+ ((ﾟωﾟﾉ==3) +'_') [ﾟｰﾟ] + ",
    "((ﾟДﾟ) +'_') [(ﾟｰﾟ)+(ﾟｰﾟ)]+ ((ﾟｰﾟ==3) +'_') [ﾟΘﾟ]+",
    "((ﾟｰﾟ==3) +'_') [(ﾟｰﾟ) - (ﾟΘﾟ)]+(ﾟДﾟ) ['c']+",
    "((ﾟДﾟ)+'_') [(ﾟｰﾟ)+(ﾟｰﾟ)]+ (ﾟДﾟ) ['o']+",
    "((ﾟｰﾟ==3) +'_') [ﾟΘﾟ];(ﾟДﾟ) ['_'] =(o^_^o) [ﾟoﾟ] [ﾟoﾟ];",
    "(ﾟεﾟ)=((ﾟｰﾟ==3) +'_') [ﾟΘﾟ]+ (ﾟДﾟ) .ﾟДﾟﾉ+",
    "((ﾟДﾟ)+'_') [(ﾟｰﾟ) + (ﾟｰﾟ)]+((ﾟｰﾟ==3) +'_') [o^_^o -ﾟΘﾟ]+",
    "((ﾟｰﾟ==3) +'_') [ﾟΘﾟ]+ (ﾟωﾟﾉ +'_') [ﾟΘﾟ]; ",
    "(ﾟｰﾟ)+=(ﾟΘﾟ); (ﾟДﾟ)[ﾟεﾟ]='\\\\'; ",
    "(ﾟДﾟ).ﾟΘﾟﾉ=(ﾟДﾟ+ ﾟｰﾟ)[o^_^o -(ﾟΘﾟ)];",
    "(oﾟｰﾟo)=(ﾟωﾟﾉ +'_')[c^_^o];",
    "(ﾟДﾟ) [ﾟoﾟ]='\\\"';",
    "(ﾟДﾟ) ['_'] ( (ﾟДﾟ) ['_'] (ﾟεﾟ+",
    "(ﾟДﾟ)[ﾟoﾟ]+ ",
];

const AA: [&str; 16] = [
    "(c^_^o)",
    "(ﾟΘﾟ)",
    "((o^_^o) - (ﾟΘﾟ))",
    "(o^_^o)",
    "(ﾟｰﾟ)",
    "((ﾟｰﾟ) + (ﾟΘﾟ))",
    "((o^_^o) +(o^_^o))",
    "((ﾟｰﾟ) + (o^_^o))",
    "((ﾟｰﾟ) + (ﾟｰﾟ))",
    "((ﾟｰﾟ) + (ﾟｰﾟ) + (ﾟΘﾟ))",
    "(ﾟДﾟ) .ﾟωﾟﾉ",
    "(ﾟДﾟ) .ﾟΘﾟﾉ",
    "(ﾟДﾟ) ['c']",
    "(ﾟДﾟ) .ﾟｰﾟﾉ",
    "(ﾟДﾟ) .ﾟДﾟﾉ",
    "(ﾟДﾟ) [ﾟΘﾟ]",
];

pub fn encode(input: &str) -> String {
    let mut out = HEAD.join("");
    /*
            for (int i = 0; i < text.length(); i++) {
            int n = text.charAt(i);
            StringBuilder t = new StringBuilder("(ﾟДﾟ)[ﾟεﾟ]+");
            if (n <= 127) {
                char[] chars = Integer.toOctalString(n).toCharArray();
                for (char c : chars) t.append(b[Integer.parseInt(String.valueOf(c))]).append("+ ");
            } else {
                String hexString = "000" + Integer.toHexString(n);
                char[] chars = hexString.substring(hexString.length() - 4).toCharArray();
                t.append("(oﾟｰﾟo)+ ");
                for (char c : chars) t.append(b[Integer.parseUnsignedInt(String.valueOf(c), 16)]).append("+ ");
            }
            r.append(t);
        }
        r.append("(ﾟДﾟ)[ﾟoﾟ]) (ﾟΘﾟ)) ('_');");


    */
    /*
    for n in input.as_bytes() {
        out.push_str("(ﾟДﾟ)[ﾟεﾟ]+");
        if *n < 128 {
            let c = format!("{}", n).as_bytes();
        }
        else {
            let hex = format!("000{x}", n);
        }
    }
    */
    return out;
}

pub fn decode(input: &str) -> String {
    unimplemented!()
}


#[test]
fn test() {
    let out = encode("alert(1)");

    println!("{}", out)
}

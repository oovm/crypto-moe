use crate::{utils::Aligned, v1::decoder::cycle_xor};
use flate2::write::DeflateDecoder;
use std::io::Write;

pub fn decompress<'a>(input: Vec<u8>) -> Result<String, String> {
    let mut compressed = input;
    cycle_xor(&mut compressed);
    cycle_xor(&mut compressed);

    let mut writer = Vec::new();
    let mut deflater = DeflateDecoder::new(writer);
    if let Err(e) = deflater.write_all(&compressed[..]) {
        return Err(e.to_string());
    }
    writer = match deflater.finish() {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    match String::from_utf8(writer) {
        Ok(o) => Ok(o),
        Err(e) => return Err(e.to_string()),
    }
}

/// doc me
pub fn decode(s: &str) -> String {
    let mut r = s.to_string();
    r.retain(|c| !"·".contains(c));
    let mapped = Aligned.decode(&r);
    match decompress(mapped) {
        Ok(s) => s,
        Err(e) => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn short() {
        //[219, 185, 237, 192, 239, 157, 191, 246, 92, 58, 249, 235, 244, 129, 75, 87, 116, 20, 142, 125, 188, 244, 105, 247, 183, 29, 139, 55, 222, 57, 254, 253, 218, 58, 123, 0]
        let r = decode("筱秋·玉筱清·情寒咏娅思园丽·思妲凤思云叶御安·娅心亚兮怡·兮倩玖·环思千多·思妲黎御倩凌·御俏·城寇奥凝寒叶咏怡·冰咏琰飘");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
        let r = decode("莉莎蕾·莉可·姆塔巧恩·如希格如·慕洁如曼淑多春·恩夏枝浅奥·浅樱蕴·铃如滢御如慕殇·多樱海多桂·寂墨怡沫·塔淑巧奥·涅巧阳宁");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
    }
}

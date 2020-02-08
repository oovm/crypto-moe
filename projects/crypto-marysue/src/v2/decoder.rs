use crate::{auxiliary::ALIGNED, v1::decoder::decompress};

pub fn decode(s: &str) -> String {
    let mut r = s.to_string();
    r.retain(|c| !"·".contains(c));
    let mapped = ALIGNED.decode(&r);
    match decompress(mapped) {
        Ok(s) => s,
        Err(_e) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[rustfmt::skip]
    fn short() {
        //[219, 185, 237, 192, 239, 157, 191, 246, 92, 58, 249, 235, 244, 129, 75, 87, 116, 20, 142, 125, 188, 244, 105, 247, 183, 29, 139, 55, 222, 57, 254, 253, 218, 58, 123, 0]
        let r = decode("紫浅凪寒·殇寂姬兰文陌琰·拉巧春妮·薰霞萨·舒咏·莺冰春英斯千绯·基瑞毓·琼恋悦洛·琰菲紫安");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
        let r = decode("凡莉·米霜菲霭玛秋蕾弥·妮迷音蕊琰晶寇月·颜瑰桂秀蕊滢·蕴白伊璃嘉茹妍贞·血莎妮殇凡文");
        debug_assert_eq!(r, "苟利国家生死以, 岂因祸福避趋之?");
    }
}

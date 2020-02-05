use std::collections::HashMap;
use std::ops::{Index, Rem};
use rand::seq::SliceRandom;
use convert_base::Convert; // 0.7.2

fn main() {
    let vs = vec![0, 1, 2, 3, 4];
    println!("{:?}", vs.choose(&mut rand::thread_rng()));
}

#[derive(Debug)]
pub struct SecretOrder {
    order: u64,
    set: String,
    map: HashMap<u64, Vec<char>>,
}

impl SecretOrder {
    pub fn new(codes: &str) -> Self {
        let count = codes.chars().count();
        let order = u64::pow(2, (count as f32).log2().floor() as u32);


        SecretOrder { set: codes.to_string(), order, map: Default::default() }
    }
    pub fn encode(self, v: &[u8]) -> Vec<char> {
        let mut base = Convert::new(256, self.order);
        let output = base.convert::<u8, u64>(&v.to_vec());
        self.index_vec(output)
    }
    pub fn decode(self, s: &str) -> Vec<u8> {
        unimplemented!()
    }
}

impl Index<usize> for SecretOrder {
    type Output = char;
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}

impl Index<char> for SecretOrder {
    type Output = Option<usize>;
    fn index(&self, index: char) -> &Self::Output {
        unimplemented!()
    }
}

impl Index<&str> for SecretOrder {
    type Output = Vec<usize>;
    fn index(&self, index: &str) -> &Self::Output {
        unimplemented!();
    }
}

impl SecretOrder {
    fn index_vec(&self, index: Vec<u64>) -> Vec<char> {
        index.iter().map(|i| self.set.chars().nth(*i as usize).unwrap()).collect()
        //let mut v = vec![];
        //let c = self.set.chars();
        //for i in index {
        //    v.push(c.nth(i as usize).unwrap())
        //}
        //return v;
    }
    fn index_char(&self, index: char) -> Option<usize> {
        self.set.chars().position(|c| c == index)
    }
    fn index_str(&self, index: &str) -> Vec<usize> {
        let mut dict = HashMap::new();
        let mut count = 0;
        for c in self.set.chars() {
            dict.insert(c, count % self.order as usize);
            count += 1
        }
        index.chars().map(|i| dict[&i]).collect()
    }
}


// 50 chars for each line
pub const CHAR_SET: &str = "\
丝丹丽之乐云亚仪伊优伤佳依俏倩倾兮兰冰凌凝凡凤凪利千华卿可叶吉君咏哀嘉园城基塔墨夏多奥如妍妖妙妮妲姆\
姣姬娅娜娣娥娴婉婵婷媛嫩宁安宜寂寇寒岚巧希幻幽弥彩影御心思怡恋恩悠悦情慕慧拉文斯春昭晓晗晶曦曼月朵枝\
枫柒柔格桂梅梦樱欢欣殇残毓沫泪洁洛浅海涅淑清温渺滢澜澪灵烟然燕燢爱爽玉玖玛玥玫环玲珊珍珠琉琦琪琬琰琳\
琴琼瑗瑞瑟瑰瑶瑷璃璎璐璧白百盘眉真碎离秀秋筱米素紫红纨纯纱绯缈美羽翠翼育舒舞艳艺艾芊芝芬花芳芸苏苑英\
茉茗茜茹荔荷莉莎莲莳莹莺菁菲萌萍萝萦萨落蒂蓉蓓蓝蔷蕊蕴蕾薇薰蝶融血裳语贞迷邪铃银锦阳陌雁雅雨雪霄霜霞\
霭露青静音韵颖颜风飘香馥馨魂魅魑鸢黎黛";


#[test]
fn test() {
    let o = SecretOrder::new(CHAR_SET);
    let t = "苟利国家生死以, 岂因祸福避趋之?".as_bytes();
    let output = o.encode(t);
    println!("{:#?}", output);
    unreachable!()
}
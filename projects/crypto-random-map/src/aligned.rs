//! doc me
use convert_base::Convert;
use rand::seq::SliceRandom;
use std::{collections::BTreeMap, ops::Index, str};

/// SecretAligned
#[derive(Debug)]
pub struct SecretAligned {
    order: u64,
    set: String,
    map: BTreeMap<u64, Vec<char>>,
}

impl SecretAligned {
    /// doc me
    pub fn new(codes: &str) -> Self {
        let count = codes.chars().count();
        let order = u64::pow(2, (count as f32).log2().floor() as u32) as usize;
        let mut map = BTreeMap::new();
        if order == count {
            for i in 0..order {
                map.insert(i as u64, vec![codes.chars().nth(i).unwrap()]);
            }
        }
        else {
            for i in 0..(count - order) {
                map.insert(i as u64, vec![codes.chars().nth(i).unwrap(), codes.chars().nth(i + order).unwrap()]);
            }
            for i in (count - order)..order {
                map.insert(i as u64, vec![codes.chars().nth(i).unwrap()]);
            }
        }
        SecretAligned { set: codes.to_string(), order: order as u64, map }
    }
    /// doc me
    pub fn encode(&self, v: &[u8]) -> Vec<char> {
        let mut base = Convert::new(256, self.order);
        let output = base.convert::<u8, u64>(&v.to_vec());
        self.index_vec(output)
    }
    /// doc me
    pub fn decode(&self, s: &str) -> Vec<u8> {
        let c = s.chars().collect::<Vec<char>>();
        let v = self.index_str(c);
        let mut base = Convert::new(self.order, 256);
        base.convert::<u64, u8>(&v.to_vec())
    }
}

#[allow(unused_variables)]
#[allow(unreachable_code)]
impl Index<usize> for SecretAligned {
    type Output = Option<char>;
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
impl Index<char> for SecretAligned {
    type Output = Option<usize>;
    fn index(&self, index: char) -> &Self::Output {
        unimplemented!();
        &self.set.chars().position(|c| c == index)
    }
}

impl SecretAligned {
    fn index_vec(&self, index: Vec<u64>) -> Vec<char> {
        // index.iter().map(|i| self.set.chars().nth(*i as usize).unwrap()).collect()
        let mut v = vec![];
        for i in index {
            let r = self.map[&i].choose(&mut rand::thread_rng());
            v.push(*r.unwrap())
        }
        return v;
    }
    fn index_str(&self, index: Vec<char>) -> Vec<u64> {
        let mut dict = BTreeMap::new();
        let mut count = 0;
        for c in self.set.chars() {
            dict.insert(c, count % self.order as usize);
            count += 1
        }
        index.iter().map(|i| dict[&i] as u64).collect()
    }
}

use convert_base::Convert;
use rand::seq::SliceRandom;
use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{Index, Rem},
    str,
};

#[derive(Debug)]
pub struct SecretDense {
    order: u64,
    set: String,
}

impl SecretDense {
    pub fn new(codes: &str) -> Self {
        let count = codes.chars().count();
        let order = u64::pow(2, (count as f32).log2().floor() as u32);
        let mut map = HashMap::new();
        SecretDense { set: codes.to_string(), order }
    }
    pub fn encode(&self, v: &[u8]) -> Vec<char> {
        unimplemented!();
        let mut base = Convert::new(256, self.order);
        let output = base.convert::<u8, u64>(&v.to_vec());
        self.index_vec(output)
    }
    pub fn decode(&self, s: &str) -> Vec<u8> {
        unimplemented!();
        let c = s.chars().collect::<Vec<char>>();
        let v = self.index_str(c);
        let mut base = Convert::new(self.order, 256);
        base.convert::<u64, u8>(&v.to_vec())
    }
}

impl Index<usize> for SecretDense {
    type Output = Option<char>;
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}

impl Index<char> for SecretDense {
    type Output = Option<usize>;
    fn index(&self, index: char) -> &Self::Output {
        unimplemented!();
        &self.set.chars().position(|c| c == index)
    }
}

impl SecretDense {
    fn index_vec(&self, index: Vec<u64>) -> Vec<char> {
        unimplemented!();
        // index.iter().map(|i| self.set.chars().nth(*i as usize).unwrap()).collect()
        let mut v = vec![];
        for i in index {
            let r = self.map[&i].choose(&mut rand::thread_rng());
            v.push(*r.unwrap())
        }
        return v;
    }
    fn index_str(&self, index: Vec<char>) -> Vec<u64> {
        unimplemented!();
        let mut dict = HashMap::new();
        let mut count = 0;
        for c in self.set.chars() {
            dict.insert(c, count % self.order as usize);
            count += 1
        }
        index.iter().map(|i| dict[&i] as u64).collect()
    }
}

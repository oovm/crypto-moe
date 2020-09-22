//! doc me
use convert_base::Convert;
use std::{ops::Index, str};

/// doc me
#[derive(Debug)]
pub struct SecretDense {
    order: u64,
    set: String,
}

impl SecretDense {
    /// doc me
    pub fn new(codes: &str) -> Self {
        let order = codes.chars().count() as u64;
        SecretDense { set: codes.to_string(), order }
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
impl Index<usize> for SecretDense {
    type Output = Option<char>;
    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!()
    }
}
#[allow(unused_variables)]
#[allow(unreachable_code)]
impl Index<char> for SecretDense {
    type Output = Option<usize>;
    fn index(&self, index: char) -> &Self::Output {
        unimplemented!()
    }
}

impl SecretDense {
    fn index_vec(&self, index: Vec<u64>) -> Vec<char> {
        let mut result = vec![];
        for i in index {
            let c = self.set.chars().nth(i as usize).unwrap();
            result.push(c)
        }
        return result;
    }
    fn index_str(&self, index: Vec<char>) -> Vec<u64> {
        let mut result = vec![];
        for i in index {
            let u = self.set.chars().position(|c| c == i).unwrap();
            result.push(u as u64)
        }
        return result;
    }
}

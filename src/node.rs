use std::collections::BTreeMap;
use sha2::{Sha512_256, Digest};

// const CHUNK_SIZE: usize = 256;

pub enum Node {
    Leaf { data: BTreeMap<Vec<u8>, Vec<u8>>, hash: String },
    Branch { children: Vec<Node> },
}

impl Node {
    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        match self {
            Node::Leaf { data, hash: _ } => data.get(&key).cloned(),
            Node::Branch { children } => {
                todo!()
                // let chunk = &key[..CHUNK_SIZE];
                // let child = children.get(chunk[0] as usize)?;
                // child.get((&key[CHUNK_SIZE..]).to_vec())
            }
        }
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        match self {
            Node::Leaf { data, hash: _ } => {
                data.insert(key, value);
                self.hash();
            }
            Node::Branch { children } => {
                todo!()
                // let chunk = &key[..CHUNK_SIZE];
                // let index = chunk[0] as usize;
                // if children.len() <= index {
                //     children.resize_with(index + 1, || Node::Leaf {
                //         data: BTreeMap::new(),
                //         hash: "".to_string(),
                //     });
                // }
                // let child = &mut children[index];
                // child.put(key[CHUNK_SIZE..].to_vec(), value);
                // let hash = child.hash();
                // println!("hash: {}", hash);
            }
        }
    }

    fn hash(&mut self) -> String {
        match self {
            Node::Leaf { data, hash } => {
                let mut hasher = Sha512_256::new();
                let values: Vec<u8> = data.values().flat_map(|v| v.iter()).cloned().collect();
                hasher.update(values.as_slice());
                *hash = format!("{:x}", hasher.finalize());
                hash.to_string()
            }
            Node::Branch { children: _ } => todo!(),
        }
    }
}
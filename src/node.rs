use crate::kvstore::KVStore;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Node {
    Leaf {
        digest: String,
        data: BTreeMap<Vec<u8>, Vec<u8>>,
    },
    Branch {
        digest: String,
        children: Vec<Node>,
    },
}

impl KVStore<Vec<u8>, Vec<u8>> for Node {
    fn new() -> Self {
        Self::Leaf {
            digest: "".to_string(),
            data: BTreeMap::new(),
        }
    }

    fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        match self {
            Node::Leaf { data, .. } => data.get(&key).cloned(),
            Node::Branch { children, .. } => {
                let mut result = None;
                for child in children {
                    result = child.get(key.clone());
                    if result.is_some() {
                        break;
                    }
                }
                result
            }
        }
    }

    fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        match self {
            Node::Leaf { data, .. } => {
                data.insert(key.clone(), value);
            }
            Node::Branch { children, .. } => {
                for child in children {
                    child.put(key.clone(), value.clone());
                }
            }
        }
        self.calc_hash();
    }
}

impl Node {
    pub fn chunk(&mut self) -> Vec<Node> {
        match self {
            Node::Leaf { data, .. } => {
                let mut left = Node::Leaf {
                    digest: "".to_string(),
                    data: BTreeMap::new(),
                };
                let mut right = Node::Leaf {
                    digest: "".to_string(),
                    data: BTreeMap::new(),
                };
                let mut i = 0;
                for (key, value) in data {
                    if i % 2 == 0 {
                        left.put(key.clone(), value.clone());
                    } else {
                        right.put(key.clone(), value.clone());
                    }
                    i += 1;
                }
                vec![left, right]
            }
            Node::Branch { .. } => todo!(),
        }
    }

    fn calc_hash(&mut self) -> String {
        let mut hasher = Sha256::new();
        match self {
            Node::Leaf { data, digest } => {
                for (key, value) in data {
                    hasher.update(key);
                    hasher.update(value);
                }
                *digest = format!("{:x}", hasher.finalize());
                return digest.clone();
            }
            Node::Branch {
                children,
                digest,
            } => {
                for child in children {
                    let (Node::Leaf {
                        digest: child_hash, ..
                    }
                    | Node::Branch {
                        digest: child_hash, ..
                    }) = child;
                    hasher.update(child_hash);
                }
                *digest = format!("{:x}", hasher.finalize());
                return digest.clone();
            }
        }
    }
}

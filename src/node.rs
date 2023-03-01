use std::collections::BTreeMap;

use crate::kvstore::KVStore;

#[derive(Debug)]
pub enum Node<K, V> {
    Leaf {
        hash: String,
        data: BTreeMap<K, V>,
    },
    Branch {
        hash: String,
        children: Vec<Node<K, V>>,
    },
}

impl<K, V> KVStore<K, V> for Node<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new() -> Self {
        Self::Leaf {
            hash: "".to_string(),
            data: BTreeMap::new(),
        }
    }

    fn get(&self, key: K) -> Option<V> {
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

    fn put(&mut self, key: K, value: V) {
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
    }
}

impl<K, V> Node<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn chunk(&mut self) -> Vec<Node<K, V>> {
        match self {
            Node::Leaf { data, .. } => {
                let mut left = Node::Leaf {
                    hash: "".to_string(),
                    data: BTreeMap::new(),
                };
                let mut right = Node::Leaf {
                    hash: "".to_string(),
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
            Node::Branch { .. } => todo!()
        }
    }
}
// mod prolly_tree;
// mod node;

use std::collections::{HashMap, BTreeMap};
use core::hash::Hash;


trait KVStore<K, V> {
    fn new() -> Self;
    fn get(&self, key: K) -> Option<V>;
    fn put(&mut self, key: K, value: V);
}

#[derive(Debug)]
struct MemoryStore<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> KVStore<K, V> for MemoryStore<K, V> 
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: K) -> Option<V> {
        self.data.get(&key).cloned()
    }

    fn put(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
}

// struct Commit {
//     hash: String,
//     parents: Vec<String>,
//     tree: ProllyTree<Vec<u8>, Vec<u8>>,
// }


#[derive(Debug)]
struct ProllyTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> ProllyTree<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    fn new() -> Self {
        Self {
            root: Node::Leaf {
                hash: "".to_string(),
                data: BTreeMap::new(),
            },
        }
    }

    fn get(&self, key: K) -> Option<V> {
        self.root.get(key)
    }

    fn put(&mut self, key: K, value: V) {
        self.root.put(key, value);
    }
}

#[derive(Debug)]
enum Node<K, V> {
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
    fn split(&mut self) {
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
                *self = Node::Branch {
                    hash: "".to_string(),
                    children: vec![left, right],
                };
            }
            Node::Branch { children, .. } => {
                for child in children {
                    child.split();
                }
            }
        }
    }
}


fn main() {
    let mut storage: MemoryStore<Vec<u8>, Vec<u8>> = MemoryStore::new();
    for i in 0..4 {
        let key = format!("key{}", i.to_string());
        let value = format!("value{}", i.to_string());
        storage.put(key.into(), value.into());
    }
    println!("Storage {:?}", storage);



    // let mut tree = prolly_tree::ProllyTree::new();
    // tree.put(vec![2, 4], vec![8, 9]);
    // println!("Testing {:?}", tree.get(vec![2, 4]).unwrap());
}

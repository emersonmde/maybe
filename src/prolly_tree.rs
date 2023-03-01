use std::collections::BTreeMap;
use crate::node::Node;


pub struct ProllyTree {
    root: Node,
}

impl ProllyTree {
    pub fn new() -> Self {
        Self {
            root: Node::Leaf {
                data: BTreeMap::new(),
                hash: "".to_string(),
            },
        }
    }

    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        self.root.get(key)
    }

    pub fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.root.put(key, value);
    }
}




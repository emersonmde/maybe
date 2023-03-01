mod prolly_tree;
mod node;

// const MAX_KEY_SIZE: usize = 256;

// #[derive(Debug)]
// struct ProllyTree {
//     root: Node,
// }


// impl ProllyTree {
//     fn new() -> Self {
//         Self {
//             root: Node::Leaf {
//                 data: BTreeMap::new(),
//             },
//         }
//     }

//     fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
//         self.root.get(key)
//     }

//     fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
//         self.root.put(key, value);
//     }
// }

// #[derive(Debug)]
// enum Node {
//     Leaf { data: BTreeMap<Vec<u8>, Vec<u8>> },
//     Branch { children: Vec<Node> },
// }

// impl Node {
//     fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
//         match self {
//             Node::Leaf { data } => data.get(&key).cloned(),
//             Node::Branch { children } => {
//                 let chunk = &key[..MAX_KEY_SIZE];
//                 let child = children.get(chunk[0] as usize)?;
//                 child.get((&key[MAX_KEY_SIZE..]).to_vec())
//             }
//         }
//     }

//     fn put(&mut self, key: Vec<u8>, value: Vec<u8>) {
//         match self {
//             Node::Leaf { data } => {
//                 data.insert(key, value);
//             }
//             Node::Branch { children } => {
//                 // let chunk = &key[..MAX_KEY_SIZE];
//                 // let index = chunk[0] as usize;
//                 // if children.len() <= index {
//                 //     children.resize_with(index + 1, || Node::Leaf {
//                 //         data: BTreeMap::new(),
//                 //     });
//                 // }
//                 // let child = &mut children[index];
//                 // child.put(key[MAX_KEY_SIZE..].to_vec(), value);
//             }
//         }
//     }
// }


fn main() {
    let mut tree = prolly_tree::ProllyTree::new();
    tree.put(vec![2, 4], vec![8, 9]);
    println!("Testing {:?}", tree.get(vec![2, 4]).unwrap());
    println!("Hello, world!");
}

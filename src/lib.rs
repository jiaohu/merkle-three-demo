use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug)]
struct MerkleTree {
    root: Option<Node>,
}

#[derive(Debug, Clone, Hash)]
enum Node {
    Leaf { hash: u64, data: String },
    Branch { hash: u64, left: Box<Node>, middle: Box<Node>, right: Box<Node> },
}

impl Node {
    fn get_hash(&self) -> u64 {
        match self {
            Node::Leaf { hash, .. } => *hash,
            Node::Branch { hash, .. } => *hash,
        }
    }
}

#[derive(Debug)]
struct Proof {
    target_hash: u64,
    proof_hashes: Vec<u64>,
    proof_directions: Vec<Direction>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Middle,
    Right,
}

impl MerkleTree {
    fn new(data: Vec<String>) -> Self {
        let leaves = data.into_iter()
            .map(|d| {
                let mut hasher = DefaultHasher::new();
                d.hash(&mut hasher);
                let hash = hasher.finish();
                Node::Leaf { hash, data: d }
            })
            .collect::<Vec<Node>>();

        let root = MerkleTree::build_tree(leaves);
        MerkleTree { root }
    }

    fn build_tree(mut nodes: Vec<Node>) -> Option<Node> {
        if nodes.is_empty() {
            return None;
        }

        while nodes.len() > 1 {
            let mut new_level = Vec::new();

            while !nodes.is_empty() {
                let left = nodes.pop().unwrap();
                let middle = nodes.pop().unwrap_or_else(|| left.clone());
                let right = nodes.pop().unwrap_or_else(|| left.clone());

                let mut hasher = DefaultHasher::new();

                left.hash(&mut hasher);
                middle.hash(&mut hasher);
                right.hash(&mut hasher);
                let hash = hasher.finish();

                let branch = Node::Branch {
                    hash,
                    left: Box::new(left),
                    middle: Box::new(middle),
                    right: Box::new(right),
                };

                new_level.push(branch);
            }

            nodes = new_level;
        }

        nodes.pop()
    }

    fn root_hash(&self) -> Option<u64> {
        self.root.as_ref().map(|n| match n {
            Node::Leaf { hash, .. } => *hash,
            Node::Branch { hash, .. } => *hash,
        })
    }

    fn calculate_hash(&self, data: &str) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        Some(hasher.finish())
    }


    fn proof(&self, data: &str) -> Option<Proof> {
        let target_hash = self.calculate_hash(data)?;

        let mut proof_hashes = Vec::new();
        let mut proof_directions = Vec::new();

        let result = self.proof_recursion(self.root.as_ref()?, target_hash, &mut proof_hashes, &mut proof_directions);

        if result {
            Some(Proof {
                target_hash,
                proof_hashes,
                proof_directions,
            })
        } else {
            None
        }
    }

    fn proof_recursion(&self, node: &Node, target_hash: u64, proof_hashes: &mut Vec<u64>, proof_directions: &mut Vec<Direction>) -> bool {
        match node {
            Node::Leaf { hash, .. } => *hash == target_hash,
            Node::Branch { hash, left, middle, right } => {
                if *hash == target_hash {
                    true
                } else {
                    let found_left = self.proof_recursion(left, target_hash, proof_hashes, proof_directions);
                    let found_middle = self.proof_recursion(middle, target_hash, proof_hashes, proof_directions);
                    let found_right = self.proof_recursion(right, target_hash, proof_hashes, proof_directions);

                    if found_left {
                        proof_hashes.push(left.get_hash());
                        proof_directions.push(Direction::Left);
                        true
                    } else if found_middle {
                        proof_hashes.push(middle.get_hash());
                        proof_directions.push(Direction::Middle);
                        true
                    } else if found_right {
                        proof_hashes.push(right.get_hash());
                        proof_directions.push(Direction::Right);
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MerkleTree;

    #[test]
    fn test_1() {
        let data = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("Merkle"),
            String::from("Tree"),
        ];

        let merkle_tree = MerkleTree::new(data);
        println!("Root Hash: {:?}", merkle_tree.root_hash());
        println!("Merkle Tree: {:?}", merkle_tree);

        let proof = merkle_tree.proof("Tree");
        println!("{:?}", proof);
    }
}

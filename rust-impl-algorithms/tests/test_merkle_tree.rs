use rust_impl_algorithms::algos::merkle_tree::MerkleTree;
use rust_impl_algorithms::traits::TreeElement;
use std::ops::Add;
use std::hash::Hash;
use std::fmt::Debug;



#[cfg(test)]
mod tests {
    use rust_impl_algorithms::traits::Tree;
    use super::*;

    #[derive(Clone, Hash, Debug)]
    pub struct Int32(pub i32);

    #[test]
fn test_create_merkle_tree() {
    let data = vec![1, 2, 3, 4];
    let tree = MerkleTree::build(data);
    let root = tree.root();
    
    fn verify_node_hash(tree: &MerkleTree<i32>, node_index: usize) {
        let node = &tree.nodes[node_index];
        if let Some(node) = node {
            if !node.children.is_empty() {
                let children_hash = node.children.iter()
                    .map(|&child_idx| tree.nodes[child_idx].as_ref().unwrap().hash_value)
                    .reduce(|a, b| a.add(b))
                    .unwrap();
                assert_eq!(node.hash_value, children_hash, "Hash mismatch for node at index {}", node_index);
                
                // Recursively verify children
                for &child_idx in &node.children {
                    verify_node_hash(tree, child_idx);
                }
            }
        }
    }

    // Start verification from the root
    verify_node_hash(&tree, 0);
    }

}



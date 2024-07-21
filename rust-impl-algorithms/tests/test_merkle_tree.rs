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

    #[cfg(test)]

    #[test]
    fn test_insert_merkle_tree() {
        // Create a new MerkleTree with initial elements
        let mut tree = MerkleTree::build(vec![1, 2, 3, 4]);
        let initial_root_hash = tree.root().hash_value.clone();

        // Insert a new element
        tree.insert(5);


        
        // Verify the structure of the tree
        fn verify_tree_structure(tree: &MerkleTree<i32>) {
                for (i, node) in tree.nodes.iter().enumerate() {
                    if let Some(node) = node {
                        if !node.children.is_empty() {
                            let children_hash = node.children.iter()
                            .filter_map(|&child_idx| tree.nodes.get(child_idx).and_then(|n| n.as_ref()).map(|n| n.hash_value.clone()))
                            .reduce(|a, b| a.add(b))
                            .unwrap();
                        assert_eq!(node.hash_value, children_hash, "Hash mismatch for node at index {}", i);
                    }
                }
            }
        }
    
        verify_tree_structure(&tree);
        
        // Verify that the new element is present
        let leaf_nodes: Vec<_> = tree.nodes.iter()
        .filter_map(|n| n.as_ref())
        .filter(|n| n.children.is_empty())
        .collect();
        
        // Verify that the root hash has changed
        assert_ne!(tree.root().hash_value, initial_root_hash, "Root hash should change after insertion");
        assert_eq!(leaf_nodes.len(), 5, "There should be 5 leaf nodes");
        assert!(leaf_nodes.iter().any(|n| n.value == Some(5)), "The inserted value 5 should be present in the leaf nodes");
    }

    #[test]
    fn test_delete_merkle_tree() {
        let mut tree = MerkleTree::build(vec![1, 2, 3, 4]);
        let initial_root_hash = tree.root().hash_value.clone();
        tree.delete(&4);
        assert_eq!(tree.n_leafs, 3, "Tree size should be 3 after deletion {:?}", tree.nodes);
        assert_ne!(tree.root().hash_value, initial_root_hash, "Root hash should change after deletion");
    }

}





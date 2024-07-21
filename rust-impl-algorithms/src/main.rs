mod algos;
mod traits;
mod utils;
use std::sync::Arc;
use traits::{TreeElement, Tree, TreeNode};
use algos::merkle_tree::MerkleTree;
use rayon::iter::IntoParallelRefIterator;
use std::hash::Hash;

fn get_bottom_nodes<T: TreeElement>(nodes: Vec<Option<TreeNode<T>>>) -> Vec<TreeNode<T>> {
    nodes.into_iter()
    .filter_map(|n| n)
    .filter(|child| child.children.is_empty())
    .collect()
}

fn test_delete_merkle_tree() {
    let mut tree = MerkleTree::build(vec![1, 2, 3, 4]);
    let initial_root_hash = tree.root().hash_value.clone();
    tree.delete(&4);
    assert_eq!(tree.n_leafs, 3, "Tree size should be 3 after deletion {:?}", tree.nodes);
    assert_ne!(tree.root().hash_value, initial_root_hash, "Root hash should change after deletion");
}

fn main() {
    test_delete_merkle_tree();
}
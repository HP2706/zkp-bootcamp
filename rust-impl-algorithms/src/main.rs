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

fn check_build(){
    let elms = vec![1, 2, 3, 4];
    let mut tree = MerkleTree::build(elms);
    let elms = get_bottom_nodes::<i32>(tree.nodes.clone());

    tree.insert(5);
    let elms = get_bottom_nodes::<i32>(tree.nodes.clone());
}

fn main() {
    check_build();
}
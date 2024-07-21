mod algos;
mod traits;
mod utils;
use std::sync::Arc;
use traits::{TreeElement, Tree, Concat};
use algos::merkle_tree::MerkleTree;
use rayon::iter::IntoParallelRefIterator;
use std::hash::Hash;


fn check_build(){
    let elms = vec![1, 2];
    let tree = MerkleTree::build(elms);
    println!("{:?}", tree.root());
}

fn main() {
    check_build();
}

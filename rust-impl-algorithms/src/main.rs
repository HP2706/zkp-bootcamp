mod algos;
mod traits;
mod utils;
use std::sync::Arc;
use traits::{TreeElement, Tree, Concat};
use algos::merkle_tree::MerkleTree;
use rayon::iter::IntoParallelRefIterator;
use std::hash::Hash;


fn check_build(){
    let elms = vec![1, 2, 3, 4];
    let tree = MerkleTree::build(elms);
    println!("{:?}", tree.root());
    println!("{:?}", tree.nodes);
    println!("branching factor {:?}", tree.branching_factor);
}

fn main() {
    check_build();
}

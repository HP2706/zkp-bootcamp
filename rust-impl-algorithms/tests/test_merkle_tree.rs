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
        let data = vec![1,2,3,4];
        let tree = MerkleTree::build(data);
        let root = tree.root();
    }

}



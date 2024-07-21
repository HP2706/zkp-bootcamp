use std::fmt::Debug;
use std::hash::{Hash, Hasher, DefaultHasher};
use std::usize;
use std::ops::Add;

pub trait Concat<T> {
    fn concat(self, other: T) -> Self;
}

pub trait TreeElement: Clone + Hash + Debug + Send + Sync {}

impl<T: Clone + Hash + Debug + Send + Sync> TreeElement for T {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub struct HashValue {
    value : [u8; 8],
    
}

impl From<usize> for HashValue {
    fn from(value: usize) -> Self {
        HashValue { value: value.to_ne_bytes() }
    }
}

impl Add for HashValue {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_value = [0; 8];
        for i in 0..8 {
            new_value[i] = self.value[i].wrapping_add(other.value[i]);
        }
        HashValue { value: new_value }
    }
}




///  A node in the tree
///  value: the value of the node
///  hash_value: the hash of the value
///  children: the indices of the children of the node
#[derive(Debug, Clone, Default)]
pub struct TreeNode<T: TreeElement> {
    pub value: Option<T>,
    pub hash_value: HashValue,
    pub children: Vec<usize>, 
    pub idx: usize,
}

impl<T: TreeElement> TreeNode<T> {
    pub fn new(value: T, idx: usize) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        TreeNode { 
            value: Some(value), 
            hash_value: HashValue::from(hash as usize), 
            children: Vec::new(),
            idx: idx
        }
    }

    pub fn internal_add(hash : HashValue, child_idxs : Vec<usize>, idx: usize) -> Self {
        TreeNode { 
            value: None, 
            hash_value: hash,
            children: child_idxs,
            idx: idx
        }
    }
}

pub trait Tree<T> 
where
    T: TreeElement,
{
    type Builder;

    fn builder() -> Self::Builder;
    fn build(elements: Vec<T>) -> Self;

    /// Insert a new element H into the tree
    fn insert(&mut self, element: T) -> bool;

    /// Combine the hashes of multiple elements

    /// Combine n hashes into one
    fn merge_hash(elms: &Vec<&usize>) -> usize {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};
        
        elms.into_par_iter()
            .map(|elm| {
                let mut hasher = DefaultHasher::new();
                elm.hash(&mut hasher);
                hasher.finish()
            })
            .reduce_with(|a, b| {
                let mut hasher = DefaultHasher::new();
                a.hash(&mut hasher);
                b.hash(&mut hasher);
                hasher.finish()
            })
            .map(|hash| hash as usize)
            .unwrap_or_else(|| 0)
    }

    /// Get the root hash of the tree
    fn root(&self) -> &TreeNode<T>;

    /// Generate a proof for a given element
    //fn generate_proof(&self, element: &T) -> Option<Self::Proof>;

    /// Verify a proof for a given element
    //fn verify_proof(&self, element: &T, proof: &Self::Proof) -> bool;

    /// Update an existing element in the tree

    /// Delete an element from the tree
    //fn delete(&mut self, element: &T) -> bool;

    /// Check if the tree is empty
    fn is_empty(&self) -> bool;
}

pub trait TreeBuilder<T: TreeElement> {
    type Tree: Tree<T>;

    fn add_element(self, element: T) -> Self;
    fn add_elements(self, elements: Vec<T>) -> Self;
    fn set_branching_factor(self, branching_factor: usize) -> Self;
    fn build(self) -> Self::Tree;
}
use std::hash::{Hash, Hasher, DefaultHasher};
use crate::traits::{Tree, TreeElement, TreeNode, TreeBuilder, HashValue};
use rayon::prelude::*;
use std::ops::Add;

pub struct MerkleTreeBuilder<T: TreeElement> {
    elements: Vec<T>,
    branching_factor: usize,
}

impl<T: TreeElement> TreeBuilder<T> for MerkleTreeBuilder<T> {
    type Tree = MerkleTree<T>;

    fn add_element(mut self, element: T) -> Self {
        self.elements.push(element);
        self
    }

    fn add_elements(mut self, mut elements: Vec<T>) -> Self {
        self.elements.append(&mut elements);
        self
    }

    fn set_branching_factor(mut self, branching_factor: usize) -> Self {
        self.branching_factor = branching_factor;
        self
    }

    fn build(self) -> Self::Tree {
        MerkleTree::build_internal(self.elements, self.branching_factor)
    }
}

pub struct MerkleTree<T: TreeElement> {
    pub nodes: Vec<Option<TreeNode<T>>>,
    pub size: usize,
    pub branching_factor: usize,
}

impl<T: TreeElement> MerkleTree<T> {
    fn build_internal(elements: Vec<T>, branching_factor: usize) -> Self {
        let size = elements.len();
        
        // Calculate the total number of nodes needed
        let total_nodes = Self::calculate_total_nodes(size as i32, branching_factor as i32);
        let mut nodes = vec![None; total_nodes as usize];

        // Insert leaf nodes
        for (i, element) in elements.into_iter().enumerate() {
            nodes[size - 1 + i] = Some(TreeNode::new(element, size - 1 + i));
        }

        // Build internal nodes bottom-up
        for i in (0..size-1).rev() {
            let children_hashes: Vec<_> = (0..branching_factor)
                .map(|j| {
                    let child_index = i * branching_factor + j + 1;
                    nodes.get(child_index).and_then(|n| n.as_ref().map(|node| node.hash_value.clone()))
                })
                .collect();

            let merged_hash = children_hashes.into_iter()
                .filter_map(|h| h)
                .reduce(|a, b| a.add(b))
                .unwrap_or_default();

            nodes[i] = Some(TreeNode::internal_add(
                merged_hash,
                (0..branching_factor).map(|j| i * branching_factor + j + 1).collect(),
                i
            ));
        }

        MerkleTree {
            nodes,
            size,
            branching_factor,
        }
    }

    fn calculate_total_nodes(size: i32, branching_factor: i32) -> i32 {
        let depth = (size as f64).log(branching_factor as f64).ceil() as u32;
        println!("depth {:?}", depth);
        let count = (
            (branching_factor.pow(depth) - 1) / (branching_factor - 1)
        ) + size;
        println!("count {:?}", count);
        count
    }

}

impl<T> Tree<T> for MerkleTree<T> 
where
    T: TreeElement,
{
    type Builder = MerkleTreeBuilder<T>;

    fn builder() -> Self::Builder {
        MerkleTreeBuilder {
            elements: Vec::new(),
            branching_factor: 2, // Default branching factor
        }
    }

    fn build(elements: Vec<T>) -> Self {
        MerkleTree::build_internal(elements, 2)
    }

    fn root(&self) -> &TreeNode<T> {
        self.nodes[0].as_ref().expect("Tree is empty")
    }

    fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        false
    }
}
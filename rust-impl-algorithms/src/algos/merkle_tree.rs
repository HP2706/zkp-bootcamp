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
    nodes: Vec<Option<TreeNode<T>>>,
    root_idx: Option<usize>,
    size: usize,
    branching_factor: usize,
}

impl<T: TreeElement> MerkleTree<T> {
    fn build_internal(elements: Vec<T>, branching_factor: usize) -> Self {
        let size = elements.len();
        
        // Calculate depth and allocate space for all nodes
        let depth = (size as f64).log(branching_factor as f64).ceil() as u32;
        let total_nodes = (branching_factor.pow(depth + 1) - 1) / (branching_factor - 1);
        let mut nodes = vec![None; total_nodes];

        // Build leaf nodes
        let leaf_start = (total_nodes - size) / branching_factor * branching_factor;
        nodes[leaf_start..].par_iter_mut()
            .zip(elements.par_iter())
            .for_each(|(node, element)| {
                *node = Some(TreeNode::new(element.clone()));
            });

        // Build internal nodes
        for level in (0..depth).rev() {
            let level_start = (branching_factor.pow(level) - 1) / (branching_factor - 1);
            let level_end = (branching_factor.pow(level + 1) - 1) / (branching_factor - 1);
            
            nodes[level_start..level_end].par_chunks_mut(branching_factor).enumerate().for_each(|(i, nodes)| {
                let children_start = level_end + i * branching_factor;
                let children_end = children_start + branching_factor;
                
                let merged_hash = nodes
                    .par_iter()
                    .filter_map(|child| child.as_ref().map(|n| n.hash_value.clone()))
                    .reduce(|| HashValue::default(), |a, b| a.add(b));

                nodes[0] = Some(TreeNode::internal_add(
                    merged_hash,
                    (children_start..children_end).collect()
                ));
            });
        }

        return MerkleTree{
            nodes : nodes,
            root_idx: Some(0), //TODO is thic correct??
            size : size,
            branching_factor : branching_factor,
        };
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

    fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        }
        false
    }

    fn root(&self) -> &TreeNode<T> {
        match self.root_idx {
            Some(idx) => self.nodes[idx].as_ref().unwrap(),
            None => panic!("Tree is empty"),
        }
    }

}
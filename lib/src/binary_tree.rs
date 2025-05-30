use std::{cmp::Ordering};

// Thanks https://google.github.io/comprehensive-rust/smart-pointers/solution.html
// for helping me learn how to write better rust

/// Binary Tree Node
struct BinNode<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>
}
/// Binary Tree Node Stored on the Heap
type Node<T: Ord> = Box<BinNode<T>>;

/// A wrapper around a Binary Node.
/// - It may be empty
/// - It implements the binary tree operations of a node
struct Subtree<T: Ord>{
    node: Option<Node<T>>
}

/// A Binary Tree Implementation
/// - Values are ordered left to right
/// - No duplicate values
pub struct OrderedBinaryTree<T: Ord> {
    root: Subtree<T>
}

impl <T: Ord+Copy> OrderedBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: Subtree::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert_center(value);
    }

    pub fn has(&mut self, value: T) -> bool {
        self.root.has(value)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn collect(&self) -> Vec<T> {
        self.root.collect(None, &|_| true)
    }

    pub fn leaves(self) -> Vec<T> {
        self.root.collect(None, &|n: &Node<T>| n.is_leaf())
    }

    pub fn value(&self) -> Option<T> {
        match &self.root.node {
            None => None,
            Some(n) => Some(n.value)
        }
    }
}


pub struct BinaryTree<T: Ord> {
    root: Subtree<T>
}

impl <T: Ord+Copy> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: Subtree::new() }
    }

    pub fn insert_center(&mut self, value: T) {
        self.root.insert_center(value);
    }

    pub fn insert_left(&mut self, value: T) {
        self.root.insert_left(value);
    }

    pub fn insert_right(&mut self, value: T) {
        self.root.insert_right(value);
    }

    pub fn leaves(self) -> Vec<T> {
        self.root.collect(None, &|n: &Node<T>| n.is_leaf())
    }

    pub fn value(&self) -> Option<T> {
        match &self.root.node {
            None => None,
            Some(n) => Some(n.value)
        }
    }
}

impl <T: Ord+Copy> Subtree<T> {
    fn new() -> Self {
        Self { node: None }
    }

    fn insert_center(&mut self, value: T) {
        match &mut self.node {
            None => self.node = Some(BinNode::bnew(value)),
            Some(_) => panic!("Inserting Into Already Populated Node")
        }
    }

    fn insert_left(&mut self, value: T) {
        match &mut self.node {
            None => panic!("Inserting Left into Empty Node"),
            Some(n) => n.left.insert_center(value),
        }
    }

    fn insert_right(&mut self, value: T) {
        match &mut self.node {
            None => panic!("Inserting Right into Empty Node"),
            Some(n) => n.right.insert_center(value),
        }
    }

    /// Recursively insert value into tree enforcing order
    fn insert_ord(&mut self, value: T) {
        match &mut self.node {
            None => self.node = Some(BinNode::bnew(value)),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert_ord(value),
                Ordering::Equal => (),
                Ordering::Greater => n.right.insert_ord(value)
            }
        }
    }

    fn has(&mut self, value: T) -> bool {
        match &mut self.node {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => n.left.has(value),
                Ordering::Greater => n.right.has(value)
            }
        }
    }

    fn len(&self) -> usize {
        match &self.node {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len()
        }
    }

    fn collect(&self,
            out: Option<Vec<T>>, 
            predicate: &impl Fn(&Node<T>) -> bool) -> Vec<T> 
    {
        let mut result: Vec<T> = match out {
            None => Vec::<T>::new(),
            Some(x) => x
        };
        
        match &self.node {
            None => (),
            Some(n) => {
                result = n.left.collect(Some(result), predicate);
                if predicate(n) {
                    result.push(n.value);
                }
                result = n.right.collect(Some(result), predicate);
            },
        }
        return  result;
    }


}

impl <T: Ord+Copy> BinNode<T> {
    fn new(value: T) -> Self {
        Self { value: value, left: Subtree::new(), right: Subtree::new() }
    }

    fn bnew(value: T) -> Node<T> {
        Box::new(BinNode::new(value))
    }

    fn is_leaf(&self) -> bool {
        match (&self.left.node, &self.right.node) {
            (None, None)        => true,
            (None, Some(_))     => false,
            (Some(_), None)     => false,
            (Some(_), Some(_))  => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort() {
        let input: Vec<i32> = vec![8,2,1,4,3,7,6,5,9,0];
        let target: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];
        let mut tree = OrderedBinaryTree::<i32>::new();

        for i in input{
            tree.insert(i);
        }
        let result = tree.collect();

        assert_eq!(result, target);
    }
}
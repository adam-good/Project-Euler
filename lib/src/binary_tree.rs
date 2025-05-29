use std::cmp::Ordering;

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
struct BinaryTree<T: Ord> {
    root: Subtree<T>
}

impl <T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&mut self, value: T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl <T: Ord> Subtree<T> {
    fn new() -> Self {
        Self { node: None }
    }

    fn insert(&mut self, value: T) {
        match &mut self.node {
            None => self.node = Some(BinNode::bnew(value)),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => (),
                Ordering::Greater => n.right.insert(value)
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
}

impl <T: Ord> BinNode<T> {
    fn new(value: T) -> Self {
        Self { value: value, left: Subtree::new(), right: Subtree::new() }
    }

    fn bnew(value: T) -> Node<T> {
        Box::new(BinNode::new(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
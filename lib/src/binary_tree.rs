pub struct BinaryTree<T: Copy> {
    data: Vec<Option<Node<T>>>,
    // root: usize
}

struct Node<T> {
    value: T,
    left: usize,
    right: usize,
}

fn calc_left_idx(root: usize) -> usize {
    2 * root + 1
}

fn calc_right_idx(root: usize) -> usize {
    2 * root + 2
}

fn calc_parent_idx(root: usize) -> usize {
    (root - 1) / 2
}

impl <T: Copy> Node<T> {
    fn new(value: T, root: usize)  -> Self {
        Self { 
            value: value, 
            left: calc_left_idx(root), 
            right: calc_right_idx(root) 
        }
    }
}

impl <T: Copy> BinaryTree<T> {

    pub fn new() -> Self {
        Self { data: Vec::<Option<Node<T>>>::new() }
    }

    pub fn insert_center(&mut self, value: T) {
        self.data.insert(self.root, Some(value));
    }

    pub fn insert_left(&mut self, value: T) {
        let len = self.data.len();
        let idx = self.left_idx();
        if len < idx {
            let diff: usize = idx - len;
            self.data.extend(vec![None; diff]);
        }
        self.data.insert(self.left_idx(), Some(value));
    }

    pub fn insert_right(&mut self, value: T) {
        let len = self.data.len();
        let idx = self.right_idx();
        if len < idx {
            let diff: usize = idx - len;
            self.data.extend(vec![None; diff]);
        }
        self.data.insert(self.right_idx(), Some(value));
    }

    pub fn root_value(&self) -> Option<T> {
        match self.data.get(self.root) {
            None => None,
            Some(v) => *v
        }
    }

    pub fn left_value(&self) -> Option<T> {
        match self.data.get(self.left_idx()) {
            None => None,
            Some(v) => *v
        }
    }

    pub fn right_value(&self) -> Option<T> {
        match self.data.get(self.right_idx()) {
            None => None,
            Some(v) => *v
        }
    }

    fn move_root(mut self, idx: usize) -> Option<BinaryTree<T>> {
        match self.data.len() > idx {
            false => None,
            true => {
                self.root = idx;
                Some(self)
            }
        }
    }

    pub fn left(self) -> Option<BinaryTree<T>> {
        let idx = self.left_idx();
        self.move_root(idx)
    }

    pub fn right(self) -> Option<BinaryTree<T>> {
        let idx = self.right_idx();
        self.move_root(idx)
    }

    pub fn parent(self) -> Option<BinaryTree<T>> {
        match self.root {
            0 => None,
            _ => {
                let idx = self.parent_idx();
                self.move_root(idx)
            }
        }
    }

    pub fn get_root(self) -> BinaryTree<T> {
        self.move_root(0).unwrap()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_leaf(&self) -> bool {
        match (self.left_value(), self.right_value()) {
            (None, None) => true,
            _ => false            
        }
    }

    pub fn get_leaves(&self) -> Vec<T> {
        let mut leaves = Vec::<T>::new();
        for i in 1..self.len() {
            match () {
                
            }
            // let tree = self.move_root(i).unwrap();
            // if tree.is_leaf() {
            //     leaves.push(tree.root_value().unwrap());
            // }
        }

        leaves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let target: Vec<Option<i32>> = vec![1,2,3].iter()
                                        .map(|x| Some(*x))
                                        .collect();
        let mut tree = BinaryTree::<i32>::new();

        tree.insert_center(1);
        tree.insert_left(2);
        tree.insert_right(3);

        assert_eq!(tree.data, target)
    }

    #[test]
    fn test_get_value() {
        let data: Vec<Option<i32>> = vec![1,2,3].iter()
                                .map(|x| Some(*x))
                                .collect();
        let tree = BinaryTree{ data: data, root: 0 };

        assert_eq!(tree.root_value().unwrap(), 1);
        assert_eq!(tree.left_value().unwrap(), 2);
        assert_eq!(tree.right_value().unwrap(), 3);
    }

    #[test]
    fn test_move_root() {
        let data: Vec<Option<i32>> = vec![1,2,3,4,5].iter()
                                        .map(|x| Some(*x))
                                        .collect();
        let tree = BinaryTree{ data: data, root: 0 };
        let target: i32 = 3;

        let new_root = tree.right_idx();
        let tree = tree.move_root(new_root).unwrap();
        let result = tree.root_value().unwrap();

        assert_eq!(result, target);
    }
}
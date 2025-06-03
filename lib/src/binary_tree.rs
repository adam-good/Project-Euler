pub struct BinaryTree<T: Copy> {
    data: Vec<T>,
    root: usize
}

impl <T: Copy> BinaryTree<T> {
    fn left_idx(&self) -> usize {
        2 * self.root + 1
    }

    fn right_idx(&self) -> usize {
        2 * self.root + 2
    }

    fn parent_idx(&self) -> usize {
        (self.root - 1) / 2
    }

    pub fn new() -> Self {
        Self { data: Vec::<T>::new(), root: 0 }
    }

    pub fn insert_center(&mut self, value: T) {
        self.data.insert(self.root, value);
    }

    pub fn insert_left(&mut self, value: T) {
        self.data.insert(self.left_idx(), value);
    }

    pub fn insert_right(&mut self, value: T) {
        self.data.insert(self.right_idx(), value);
    }

    pub fn root_value(&self) -> Option<T> {
        self.data.get(self.root).copied()
    }

    pub fn left_value(&self) -> Option<T> {
        self.data.get(self.left_idx()).copied()
    }

    pub fn right_value(&self) -> Option<T> {
        self.data.get(self.right_idx()).copied()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let target: Vec<i32> = vec![1,2,3];
        let mut tree = BinaryTree::<i32>::new();

        tree.insert_center(1);
        tree.insert_left(2);
        tree.insert_right(3);

        assert_eq!(tree.data, target)
    }

    #[test]
    fn test_get_value() {
        let data: Vec<i32> = vec![1,2,3];
        let tree = BinaryTree{ data: data, root: 0 };

        assert_eq!(tree.root_value().unwrap(), 1);
        assert_eq!(tree.left_value().unwrap(), 2);
        assert_eq!(tree.right_value().unwrap(), 3);
    }

    #[test]
    fn test_move_root() {
        let data: Vec<i32> = vec![1,2,3,4,5];
        let tree = BinaryTree{ data: data, root: 0 };
        let target: i32 = 3;

        let new_root = tree.right_idx();
        let tree = tree.move_root(new_root).unwrap();
        let result = tree.root_value().unwrap();

        assert_eq!(result, target);
    }
}
// This is an educational exercise based on 
// https://rust-unofficial.github.io/too-many-lists/index.html
// 
// Probably not useful, but should be educational

use core::net;
use std::{io::Empty, mem};

pub struct List {
    head: Link
}

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

impl Node {
    fn new(value: i32) -> Box<Self> {
        Box::new(
            Node { elem: value, next: Link::Empty }
        )
    }
}

impl Link {
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) -> () {
        let new_node = Box::new(Node {
            elem: value,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(n) => {
                self.head = n.next;
                Some(n.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_test_pop_empty() {
        let mut list = List::new();
        let result = list.pop();

        assert_eq!(result, None);
    }

    #[test]
    fn list_test_pop_some() {
        let mut list = List {
            head: Link::More(Box::new(
                Node { elem: 5, next: Link::Empty }
            ))
        };
        let result = list.pop().unwrap();

        assert_eq!(result, 5);
    }

    #[test]
    fn list_test_push() {
        let mut list = List::new();
        list.push(5);

        let result = match list.head {
            Link::Empty => None,
            Link::More(n) => Some(n.elem)
        };

        assert_eq!(result, Some(5));
    }
}
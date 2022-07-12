pub struct I32Range {
    pub start: i32,
    pub end: i32,
}

impl Iterator for I32Range {
    type Item = i32;
    fn next (&mut self) -> Option<i32> {
        if self.start >= self.end {
            return None;
        }

        let result = Some(self.start);
        self.start += 1;
        result
    }
}



// binary tree iterator implementation
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

use self::BinaryTree::*;

// `BinaryTree` symmetrical traversal state
pub struct TreeIter<'a, T: 'a> {
    pub unvisited: Vec<&'a TreeNode<T>>
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

// implementation in the BinaryTree type iter method that returns an iterator 
// to traverse the tree
impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(self);
        iter
    }
}

// implement the IntoIterator characteristic for a shared tree reference using
// calls to BinaryTree::iter:
impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = TreeIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Finally, in the Iterator implementation, we get to the actual tree traversal. 
// As with the iter method of the BinaryTree type, the behavior of the next 
// iterator method is determined by stack rules:
impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };
        self.push_left_edge(&node.right);

        Some(&node.element)
    }
}
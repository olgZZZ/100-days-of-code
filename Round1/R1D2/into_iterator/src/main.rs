//Implementing custom iterators
mod my_iterators;
// use std::fmt::Binary;

use my_iterators::*;
use self::BinaryTree::*;
fn main() {
    let mut pi = 0.0;
    let mut numerator = 1.0;

    for k in (I32Range { start: 0, end: 14 }) {
        pi += numerator / (2*k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);

    println!("Pi = {}", pi);







    // using for loop to traverse BinaryTree

    fn make_node<T>(left: BinaryTree<T>, element: T, right: BinaryTree<T>) -> BinaryTree<T> {
        NonEmpty(Box::new(TreeNode { left, element, right }))
    }

    let subtree_l = make_node(Empty, "mecha", Empty);
    let subtree_rl = make_node(Empty, "droid", Empty);
    let subtree_r = make_node(subtree_rl, "robot", Empty);
    let tree = make_node(subtree_l, "Jaeger", subtree_r);

    let mut v = Vec::new();
    for kind in &tree {
        v.push(*kind);
    }

    println!("v - {:?}", v);

} 





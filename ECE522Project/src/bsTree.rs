//! Binary search tree
//!
//! You can generate a binary search tree, and insert or delete nodes.

use std::cell::RefCell;
use std::cmp::max;
use std::fmt;
use std::rc::Rc;

pub use crate::commonTrait::{CommonTreeNodeTrait, CommonTreeTrait};

#[derive(Clone, Debug, PartialEq)]
enum NodeDirection {
    Left,
    Right,
}

/// Structure of BSTree
#[derive(Clone, Debug, PartialEq)]
pub struct BSTree<T: Ord + Copy + fmt::Debug> {
    root: OptionBSTreeNode<T>,
}

/// Node struct for [BSTree](struct.BSTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T: Ord + Copy + fmt::Debug> {
    value: T,
    left: OptionBSTreeNode<T>,
    right: OptionBSTreeNode<T>,
}

type BSTreeNode<T> = Rc<RefCell<TreeNode<T>>>;
type OptionBSTreeNode<T> = Option<BSTreeNode<T>>;


// extend from common tree trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeTrait<T, TreeNode<T>> for BSTree<T> {
    fn get_root(&self) -> OptionBSTreeNode<T> {
        return self.root.clone();
    }
}

// extend from common tree node trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeNodeTrait<T> for TreeNode<T> {
    fn get_left(&self) -> OptionBSTreeNode<T> {
        return self.left.clone();
    }

    fn get_right(&self) -> OptionBSTreeNode<T> {
        return self.right.clone();
    }

    fn get_value(&self) -> T {
        return self.value;
    }
}

// BSTree
impl<T: Ord + Copy + fmt::Debug> BSTree<T> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    pub fn insert(&mut self, insert_value: T) {
        let root = self.get_root();
        match root {
            None => self.root = Some(Rc::new(RefCell::new(TreeNode::new(insert_value)))),
            Some(root) => TreeNode::node_insert(root, insert_value),
        }
    }

    pub fn delete(&mut self, delete_value: T) {
        let root = self.get_root();
        match root {
            None => (),
            Some(root) => {
                if root.borrow().get_value() > delete_value {
                    TreeNode::node_delete_left(root.clone(), delete_value);
                } else if root.borrow().get_value() < delete_value {
                    TreeNode::node_delete_right(root.clone(), delete_value);
                } else {
                    let left = root.borrow().get_left();
                    let right = root.borrow().get_right();
                    // if delete root and root does not has left or right
                    if left.is_none() && right.is_none() {
                        self.root = None;
                    }
                    // if delete root and root only has left
                    else if left.is_some() && right.is_none() {
                        self.root = left;
                    }
                    // if delete root and root only has right
                    else if left.is_none() && right.is_some() {
                        self.root = right;
                    }
                    // if delete root and root has left and right
                    else {
                        let min_of_right =
                            right.clone().unwrap().borrow().get_min_value_in_children();
                        self.root.clone().unwrap().borrow_mut().value = min_of_right;
                        TreeNode::node_delete_right(root, min_of_right);
                    }
                }
            }
        }
    }
}

// TreeNode
impl<T: Ord + Copy + fmt::Debug> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value: value,
            left: None,
            right: None,
        }
    }

    fn node_insert(node: BSTreeNode<T>, insert_value: T) {
        if node.borrow().get_value() > insert_value {
            let left = node.borrow().left.clone();
            match left {
                Some(left) => {
                    Self::node_insert(left, insert_value);
                }
                None => {
                    node.borrow_mut().left =
                        Some(Rc::new(RefCell::new(TreeNode::new(insert_value))));
                }
            }
        } else if node.borrow().get_value() < insert_value {
            let right = node.borrow().right.clone();
            match right {
                Some(right) => {
                    Self::node_insert(right, insert_value);
                }
                None => {
                    node.borrow_mut().right =
                        Some(Rc::new(RefCell::new(TreeNode::new(insert_value))));
                    let right = node.borrow().get_right();
                }
            }
        } else {
            return;
        }
    }

    fn node_delete_left(parent: BSTreeNode<T>, delete_value: T) {
        let curr_node = parent.borrow().get_left();
        match curr_node {
            None => (),
            Some(curr_node) => {
                if curr_node.borrow().get_value() > delete_value {
                    Self::node_delete_left(curr_node, delete_value);
                } else if curr_node.borrow().get_value() < delete_value {
                    Self::node_delete_right(curr_node, delete_value);
                } else {
                    let left_node = curr_node.borrow_mut().get_left();
                    let right_node = curr_node.borrow_mut().get_right();
                    // 1. current node has two children
                    // if current node has two children, then recursively replace it with the min value of right
                    // delete the min value of right in the right tree
                    // the goal is to make the problem to be the case where current node has only one child
                    if left_node.is_some() && right_node.is_some() {
                        let min_of_right = right_node
                            .clone()
                            .unwrap()
                            .borrow()
                            .get_min_value_in_children();
                        curr_node.borrow_mut().value = min_of_right;
                        Self::node_delete_right(curr_node, min_of_right);
                    }
                    // 2. current node has no child
                    else if left_node.is_none() && right_node.is_none() {
                        parent.borrow_mut().left = None;
                    }
                    // 3.1 current node has one left child
                    else if left_node.is_some() && right_node.is_none() {
                        parent.borrow_mut().left = left_node;
                    }
                    // 3.2 current node has one right child
                    else {
                        parent.borrow_mut().left = right_node;
                    }
                }
            }
        }
    }

    fn node_delete_right(parent: BSTreeNode<T>, delete_value: T) {
        let curr_node = parent.borrow().get_right();
        match curr_node {
            None => (),
            Some(curr_node) => {
                if curr_node.borrow().get_value() > delete_value {
                    Self::node_delete_left(curr_node, delete_value);
                } else if curr_node.borrow().get_value() < delete_value {
                    Self::node_delete_right(curr_node, delete_value);
                } else {
                    let left_node = curr_node.borrow_mut().get_left();
                    let right_node = curr_node.borrow_mut().get_right();
                    // 1. current node has two children
                    // if current node has two children, then recursively replace it with the min value of right
                    // delete the min value of right in the right tree
                    // the goal is to make the problem to be the case where current node has only one child
                    if left_node.is_some() && right_node.is_some() {
                        let min_of_right = right_node
                            .clone()
                            .unwrap()
                            .borrow()
                            .get_min_value_in_children();
                        curr_node.borrow_mut().value = min_of_right;
                        Self::node_delete_right(curr_node, min_of_right);
                    }
                    // 2. current node has no child
                    else if left_node.is_none() && right_node.is_none() {
                        parent.borrow_mut().right = None;
                    }
                    // 3.1 current node has one left child
                    else if left_node.is_some() && right_node.is_none() {
                        parent.borrow_mut().right = left_node;
                    }
                    // 3.2 current node has one right child
                    else {
                        parent.borrow_mut().right = right_node;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert() {
        let mut bs_tree = BSTree::new();
        bs_tree.insert(1);
        bs_tree.insert(2);
        bs_tree.insert(3);
        bs_tree.insert(4);
        bs_tree.insert(5);
        bs_tree.insert(6);
        bs_tree.in_order_traversal();
        bs_tree.pre_order_traversal();


    }

    //     #[test]
    //     fn test_delete() {
    //         // Test the three different tree traversal functions.
    //         let mut tree = BSTree::new();
    //         tree.insert(0);
    //         vec![16, 8, 24, 20, 22].iter().for_each(|v| {
    //             tree.insert(*v);
    //         });

    //         let root = tree.root.clone().unwrap();
    //         tree.delete(16);
    //         let mut container = vec![];
    //         tree.debug_preorder_traverse(root.clone(), &mut container);
    //         let result = BSTree::is_valid_red_black_tree(tree.root);
    //         assert_eq!(result, true);
    //         //  assert_eq!(container, vec![8, 0, 20, 24, 22]);
    //     }
}

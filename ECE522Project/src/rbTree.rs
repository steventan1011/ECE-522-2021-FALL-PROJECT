//! Red-black tree
//!
//! You can generate a red-black tree, and insert or delete nodes.

use std::cell::RefCell;
use std::cmp::max;
use std::fmt;
use std::rc::Rc;

pub use crate::commonTrait::{CommonTreeNodeTrait, CommonTreeTrait};

/// Color representation for the [TreeNode](struct.TreeNode.html)
/// of [RBTree](struct.RBTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    /// Red color
    Red,
    /// Black color, the root of [RBTree](struct.RBTree.html) will be set to Black
    Black,
}

#[derive(Clone, Debug, PartialEq)]
enum NodeDirection {
    Left,
    Right,
}

/// Structure of RBTree
#[derive(Clone, Debug, PartialEq)]
pub struct RBTree<T: Ord + Copy + fmt::Debug> {
    root: OptionRBTreeNode<T>,
}

/// Node struct for [RBTree](struct.RBTree.html) struct
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T: Ord + Copy + fmt::Debug> {
    /// The color of the node
    color: NodeColor,
    /// Data stored in the node
    value: T,
    parent: OptionRBTreeNode<T>,
    left: OptionRBTreeNode<T>,
    right: OptionRBTreeNode<T>,
}

type RBTreeNode<T> = Rc<RefCell<TreeNode<T>>>;
type OptionRBTreeNode<T> = Option<RBTreeNode<T>>;

/// Implementations of NodeColor
impl NodeColor {
    fn to_string(&self) -> &str {
        if self == &NodeColor::Red {
            "r"
        } else {
            "b"
        }
    }
}

// extend from common tree trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeTrait<T, TreeNode<T>> for RBTree<T> {
    fn get_root(&self) -> OptionRBTreeNode<T> {
        return self.root.clone();
    }
}

// extend from common tree node trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeNodeTrait<T> for TreeNode<T> {
    fn get_left(&self) -> OptionRBTreeNode<T> {
        return self.left.clone();
    }

    fn get_right(&self) -> OptionRBTreeNode<T> {
        return self.right.clone();
    }

    fn get_value(&self) -> T {
        return self.value;
    }
}

/// Implementations of RBTree
// RBTree
impl<T: Ord + Copy + fmt::Debug> RBTree<T> {

    /// Create a new red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTree::RBTree;
    /// let mut tree = RBTree::new();
    /// ```
    pub fn new() -> Self {
        RBTree { root: None }
    }

    /// Insert a new value to the red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.insert(1);
    /// ```
    pub fn insert(&mut self, insert_value: T) {
        let root = self.root.clone();
        self.root = match root {
            None => Some(TreeNode::set_black(Rc::new(RefCell::new(TreeNode::new(
                insert_value,
            ))))),
            Some(root) => TreeNode::node_insert(root, insert_value),
        }
    }

    /// Delete a value from the red-black Tree
    ///
    /// # Example
    ///
    /// ```
    /// use tree_collections::rbTree::RBTree;
    /// let mut tree = RBTree::new();
    /// tree.delete(1);
    /// ```
    pub fn delete(&mut self, delete_value: T) {
        let root = self.root.clone();
        match root {
            None => (),
            Some(root) => {
                let result = TreeNode::node_delete(root, delete_value);
                self.root = result;
            }
        }
    }

    pub fn pre_order_traverse(&self, node: RBTreeNode<T>, container: &mut Vec<T>) {
        container.push(node.borrow().value);
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.pre_order_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.pre_order_traverse(right.unwrap(), container);
        }
    }

    // 下面这三个之后会不要，用上面的in_order_traversal
    pub fn debug_preorder_traverse(&self, node: RBTreeNode<T>, container: &mut Vec<T>) {
        container.push(node.borrow().value);
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.debug_preorder_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.debug_preorder_traverse(right.unwrap(), container);
        }
    }

    // for testing
    pub fn inorder_traverse(&self, node: RBTreeNode<T>, container: &mut Vec<T>) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.inorder_traverse(left.unwrap(), container);
        }
        container.push(node.borrow().value);
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.inorder_traverse(right.unwrap(), container);
        }
    }

    /// Return true if the tree match propertity 5
    /// Propertity 5. Every path from any node to all of its descendent Nil nodes 
    /// has the same number of black nodes.
    pub fn is_valid_red_black_tree(root: OptionRBTreeNode<T>) -> bool {
        let result = TreeNode::calculate_black_height(root);
        match result {
            Some(_) => true,
            None => false,
        }
    }

    
    pub fn is_equal(left: OptionRBTreeNode<T>, right: OptionRBTreeNode<T>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(left), Some(right)) => {
                let left_data = left.borrow().value;
                let right_data = right.borrow().value;
                //Test if 2 trees are equal
                if left_data == right_data {
                    let left_left = left.borrow().left.clone();
                    let left_right = left.borrow().right.clone();
                    let right_left = right.borrow().left.clone();
                    let right_right = right.borrow().right.clone();
                    Self::is_equal(left_left, right_left) && Self::is_equal(left_right, right_right)
                } else {
                    false
                }
            }
        }
    }
}

/// Implementations of TreeNode
// TreeNode
impl<T: Ord + Copy + fmt::Debug> TreeNode<T> {

    /// Create a new node
    /// ,which will be called by [RBTree](struct.RBTree.html)
    fn new(value: T) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }

    // Helper function for TreeNode::node_insert
    fn new_with_parent(value: T, parent: OptionRBTreeNode<T>) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: parent,
            left: None,
            right: None,
        }
    }

    // For testing
    fn new_black_with_parent(value: T, parent: OptionRBTreeNode<T>) -> Self {
        TreeNode {
            color: NodeColor::Black,
            value: value,
            parent: parent,
            left: None,
            right: None,
        }
    }

    /// Insert data into the subtree, performs any rotations
    /// necessary to maintain banlance, and then returns the new root to this subtree
    /// , which will be called by
    /// [RBTree.insert](struct.RBTree.html#method.insert)
    fn node_insert(node: RBTreeNode<T>, insert_value: T) -> OptionRBTreeNode<T> {
        if node.borrow().value == insert_value {
            return Some(node);
        } else if node.borrow().value > insert_value {
            let left = node.borrow().left.clone();
            match left {
                Some(left) => {
                    Self::node_insert(left, insert_value);
                }
                None => {
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(
                        TreeNode::new_with_parent(insert_value, Some(node.clone())),
                    )));
                    let left = node.borrow().left.clone();
                    Self::insert_maintain_rb(left.unwrap());
                }
            }
        } else {
            let right = node.borrow().right.clone();
            match right {
                Some(right) => {
                    Self::node_insert(right, insert_value);
                }
                None => {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(
                        TreeNode::new_with_parent(insert_value, Some(node.clone())),
                    )));
                    let right = node.borrow().right.clone();
                    Self::insert_maintain_rb(right.unwrap());
                }
            }
        }
        // return the root
        return Self::get_root(node);
    }

    /// Repair the coloring from inserting into a tree.
    fn insert_maintain_rb(node: RBTreeNode<T>) {
        let parent = node.borrow().parent.clone();

        match parent {
            None => {
                // insert case 3: node is root, no parent
                // println!("insert case 3: node is root, no parent");
                Self::set_black(node);
            }
            // Check parent color
            Some(parent) => {
                // println!(
                //     "=====start at insert_maintain_rb {:#?} {:#?}",
                //     node.borrow().value,
                //     parent.clone().borrow().value
                // );
                if parent.borrow().color == NodeColor::Black {
                    // insert case 1: parent is black, do not need maintain
                    // println!("insert case 1: parent is black, do not need maintain");
                    ();
                }
                // parent is red, need maintain
                else {
                    let grand_parent = parent.borrow().parent.clone();
                    match grand_parent {
                        None => {
                            // insert case 4: grandparent is None, then parent goes to black
                            // println!("insert case 4");
                            Self::set_black(parent);
                        }
                        Some(grand_parent) => {
                            // println!(
                            //     "===== case 2 or 5 or 6 else parent at insert_maintain_rb {:#?} {:#?} {:#?}",
                            //     node.borrow().value,
                            //     parent.clone().borrow().value,
                            //     grand_parent.clone().borrow().value
                            // );
                            // let grand_parent_borrowed = grand_parent.borrow();
                            if grand_parent.borrow().color == NodeColor::Red {
                                panic!("Red violation!");
                            }
                            if Self::is_left(parent.clone()) && Self::is_left(node.clone()) {
                                let option_uncle = grand_parent.borrow().right.clone();
                                if Self::get_color(option_uncle.clone()) == NodeColor::Black {
                                    // insert case 6.1: left left && uncle is None or black
                                    println!("insert case 6.1: left left && uncle is None");
                                    Self::right_rotate(grand_parent.clone());
                                    let parent = node.borrow().parent.clone().unwrap();
                                    Self::set_black(parent.clone());
                                    let right = parent.borrow().right.clone().unwrap();
                                    Self::set_red(right.clone());
                                } else {
                                    let uncle = option_uncle.clone().unwrap();
                                    println!(
                                        "===== uncle 161 at insert_maintain_rb {:#?}",
                                        uncle.clone().borrow().value,
                                    );
                                    if uncle.borrow().color == NodeColor::Red {
                                        // insert case 2: uncle is red
                                        println!("insert case 2: uncle is red");
                                        Self::set_black(parent.clone());
                                        Self::set_black(uncle.clone());
                                        Self::set_red(grand_parent.clone());
                                        Self::insert_maintain_rb(grand_parent.clone());
                                    }
                                }
                            } else if Self::is_right(parent.clone()) && Self::is_right(node.clone())
                            {
                                let option_uncle = grand_parent.borrow().left.clone();
                                println!(
                                    "===== else self.is_right(parent.clone()) && self.is_right(node.clone() {:#?} {:#?} {:#?}",
                                    node.borrow().value,
                                    parent.clone().borrow().value,
                                    grand_parent.clone().borrow().value
                                );
                                if Self::get_color(option_uncle.clone()) == NodeColor::Black {
                                    // insert case 6.2: right right && uncle is None or black
                                    println!("insert case 6.2: right right && uncle is None");
                                    Self::left_rotate(grand_parent.clone());
                                    let parent = node.borrow().parent.clone().unwrap();
                                    Self::set_black(parent.clone());
                                    let left = parent.borrow().left.clone().unwrap();
                                    Self::set_red(left.clone());
                                } else {
                                    let uncle = option_uncle.clone().unwrap();
                                    println!(
                                        "===== uncle 197 at insert_maintain_rb {:#?}",
                                        uncle.clone().borrow().value,
                                    );
                                    // insert case 2: uncle is red
                                    println!("insert case 2: uncle is red");
                                    Self::set_black(parent.clone());
                                    Self::set_black(uncle.clone());
                                    Self::set_red(grand_parent.clone());
                                    Self::insert_maintain_rb(grand_parent.clone());
                                }
                            } else if Self::is_left(parent.clone()) && Self::is_right(node.clone())
                            {
                                let option_uncle = grand_parent.borrow().right.clone();
                                if Self::get_color(option_uncle.clone()) == NodeColor::Black {
                                    // insert case 5.1: left right  && uncle is None or black
                                    println!("case 5.1: left right  && uncle is None");
                                    println!(
                                        "===== case 5.1: {:#?} {:#?} {:#?}",
                                        node.borrow().value,
                                        parent.clone().borrow().value,
                                        grand_parent.clone().borrow().value
                                    );
                                    Self::left_rotate(parent.clone());
                                    let left_child = node.borrow().left.clone().unwrap();
                                    Self::insert_maintain_rb(left_child.clone());
                                } else {
                                    let uncle = option_uncle.clone().unwrap();
                                    println!(
                                        "===== uncle 237 at insert_maintain_rb {:#?}",
                                        uncle.clone().borrow().value,
                                    );
                                    // insert case 2: uncle is red
                                    println!("insert case 2: uncle is red");
                                    Self::set_black(parent.clone());
                                    Self::set_black(uncle.clone());
                                    Self::set_red(grand_parent.clone());
                                    Self::insert_maintain_rb(grand_parent.clone());
                                }
                            } else if Self::is_right(parent.clone()) && Self::is_left(node.clone())
                            {
                                let option_uncle = grand_parent.borrow().left.clone();
                                println!(
                                    "===== else if self.is_right(parent.clone()) && self.is_left(node.clone() {:#?} {:#?} {:#?}",
                                    node.borrow().value,
                                    parent.clone().borrow().value,
                                    grand_parent.borrow().value
                                );
                                if Self::get_color(option_uncle.clone()) == NodeColor::Black {
                                    // insert case 5.2: right left && uncle is None or black
                                    println!("insert case 5.2: right left && uncle is None");
                                    Self::right_rotate(parent.clone());
                                    let right_child = node.borrow().right.clone().unwrap();
                                    Self::insert_maintain_rb(right_child.clone());
                                } else {
                                    let uncle = option_uncle.clone().unwrap();
                                    println!(
                                        "===== uncle 282 at insert_maintain_rb {:#?}",
                                        uncle.clone().borrow().value,
                                    );
                                    // insert case 2: uncle is red
                                    println!("insert case 2: uncle is red");
                                    Self::set_black(parent.clone());
                                    Self::set_black(uncle.clone());
                                    Self::set_red(grand_parent.clone());
                                    Self::insert_maintain_rb(grand_parent.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Delete data from the subtree, performs any rotations
    /// necessary to maintain banlance, and then returns the new root to this subtree
    /// , which will be called by
    /// [RBTree.delete](struct.RBTree.html#method.delete)
    fn node_delete(node: RBTreeNode<T>, delete_value: T) -> OptionRBTreeNode<T> {
        if node.borrow().value > delete_value {
            let left_child = node.borrow().left.clone();
            if left_child.is_some() {
                Self::node_delete(left_child.unwrap(), delete_value);
            }
        } else if node.borrow().value < delete_value {
            let right_child = node.borrow_mut().right.clone();
            if right_child.is_some() {
                Self::node_delete(right_child.unwrap(), delete_value);
            }
        } else {
            let left = node.borrow_mut().left.clone();
            let right = node.borrow_mut().right.clone();
            // 1. Two children case: current node has two children
            // if current node has two children, then recursively replace it with the min value of right
            // delete the min value of right in the right tree
            // the goal is to make the problem to be the case where current node has only one child
            if left.is_some() && right.is_some() {
                let min_of_right = right.clone().unwrap().borrow().get_min_value_in_children();
                node.borrow_mut().value = min_of_right;
                Self::node_delete(right.unwrap(), min_of_right);
            }
            // current node has one child or no child
            else {
                // 2. Red case: current node is red
                // it means that current node has no child, just delete this node
                if node.borrow().color == NodeColor::Red {
                    let parent = node.borrow().parent.clone().unwrap();
                    if Self::is_left(node.clone()) {
                        parent.borrow_mut().left = None;
                    } else {
                        parent.borrow_mut().right = None;
                    }
                }
                // current node is black
                else {
                    // 3.1 Black + left red case: current node is black and left child is red, right child is None
                    // delete the current black node and move the left child to the current node place
                    if left.is_some() && right.is_none() {
                        let left = left.unwrap();
                        // TODO
                        // node.parent.child = left, left.parent = node.parent
                        let parent = node.borrow().parent.clone();
                        match parent {
                            None => {
                                left.borrow_mut().color = node.borrow().color.clone();
                                left.borrow_mut().parent = None;
                                return Some(left);
                            }
                            Some(parent) => {
                                if Self::is_left(node.clone()) {
                                    parent.borrow_mut().left = Some(left.clone());
                                    left.borrow_mut().parent = Some(parent.clone());
                                    left.borrow_mut().color = node.borrow().color.clone();
                                } else {
                                    parent.borrow_mut().right = Some(left.clone());
                                    left.borrow_mut().parent = Some(parent.clone());
                                    left.borrow_mut().color = node.borrow().color.clone();
                                }
                            }
                        }

                        // node.borrow_mut().value = left.borrow().value;
                        // node.borrow_mut().left = left.borrow().left.clone();
                        // node.borrow_mut().right = left.borrow().right.clone();
                        // if node.borrow().left.is_some() {
                        //     let left = node.borrow().left.clone().unwrap();
                        //     left.borrow_mut().parent = Some(node.clone());
                        // }
                        // if node.borrow().right.is_some() {
                        //     let right = node.borrow().right.clone().unwrap();
                        //     right.borrow_mut().parent = Some(node.clone());
                        // }
                    }
                    // 3.2 Black + right red case: current node is black and right child is red, left child is None
                    // delete the current black node and move the right child to the current node place
                    else if left.is_none() && right.is_some() {
                        let right = right.unwrap();
                        let parent = node.borrow().parent.clone();
                        match parent {
                            None => {
                                right.borrow_mut().color = node.borrow().color.clone();
                                right.borrow_mut().parent = None;
                                return Some(right);
                            }
                            Some(parent) => {
                                if Self::is_left(node.clone()) {
                                    parent.borrow_mut().left = Some(right.clone());
                                    right.borrow_mut().parent = Some(parent.clone());
                                    right.borrow_mut().color = node.borrow().color.clone();
                                } else {
                                    parent.borrow_mut().right = Some(right.clone());
                                    right.borrow_mut().parent = Some(parent.clone());
                                    right.borrow_mut().color = node.borrow().color.clone();
                                }
                            }
                        }
                        // node.borrow_mut().value = right.borrow().value;
                        // node.borrow_mut().left = right.borrow().left.clone();
                        // node.borrow_mut().right = right.borrow().right.clone();
                        // if node.borrow().left.is_some() {
                        //     let left = node.borrow().left.clone().unwrap();
                        //     left.borrow_mut().parent = Some(node.clone());
                        // }
                        // if node.borrow().right.is_some() {
                        //     let right = node.borrow().right.clone().unwrap();
                        //     right.borrow_mut().parent = Some(node.clone());
                        // }
                    }
                    // 4. Black + no children case: current node is black and has no children
                    else {
                        let parent = node.borrow().parent.clone();
                        match parent {
                            // 4.1 current node is the root, then return None
                            None => return None,
                            // 4.2 current node has parent, then call delete_maintain_rb
                            // and then delete the link between current node and its parent
                            Some(parent) => {
                                Self::delete_maintain_rb(node.clone());
                                if Self::is_left(node.clone()) {
                                    parent.borrow_mut().left = None;
                                } else {
                                    parent.borrow_mut().right = None;
                                }
                                node.borrow_mut().parent = None;
                            }
                        }
                    }
                }
            }
        }
        // return the root
        return Self::get_root(node);
    }

    /// Repair the coloring of the remaining nodes in the tree
    fn delete_maintain_rb(node: RBTreeNode<T>) {
        let parent = node.borrow().parent.clone();
        match parent {
            // delete case 2: parent is None
            // it means current node is the new root, just return
            None => return,
            Some(parent) => {
                let sibling = Self::get_sibling(node.clone());
                let direction;
                if Self::is_left(node.clone()) {
                    direction = NodeDirection::Left;
                } else {
                    direction = NodeDirection::Right;
                }
                match sibling {
                    None => return,
                    Some(sibling) => {
                        // delete case 3: sibling is red; parent, close, distant are black
                        if sibling.borrow().color == NodeColor::Red {
                            // RotateDirRoot(T, P, dir);
                            if direction == NodeDirection::Left {
                                Self::left_rotate(parent.clone());
                            } else {
                                Self::right_rotate(parent.clone());
                            }
                            Self::set_red(parent.clone());
                            Self::set_black(sibling.clone());
                            // recursive
                            Self::delete_maintain_rb(node.clone());
                        }
                        // sibling is black
                        else {
                            let option_close;
                            let option_distant;
                            if direction == NodeDirection::Left {
                                option_close = sibling.borrow().left.clone();
                                option_distant = sibling.borrow().right.clone()
                            } else {
                                option_close = sibling.borrow().right.clone();
                                option_distant = sibling.borrow().left.clone()
                            }
                            // close and distant are black
                            if Self::get_color(option_close.clone()) == NodeColor::Black
                                && Self::get_color(option_distant.clone()) == NodeColor::Black
                            {
                                // delete case 1: parent, sibling, close, distant are all black
                                if parent.borrow().color == NodeColor::Black {
                                    Self::set_red(sibling.clone());
                                    Self::delete_maintain_rb(parent.clone());
                                }
                                // delete case 4: parent is red; sibling, close, distant are black
                                else {
                                    Self::set_red(sibling.clone());
                                    Self::set_black(parent.clone());
                                }
                            }
                            // delete case 5: sibling and distant are black, close is red
                            else if Self::get_color(option_close.clone()) == NodeColor::Red
                                && Self::get_color(option_distant.clone()) == NodeColor::Black
                            {
                                // RotateDir(S,1-dir);  S is never the root
                                if direction == NodeDirection::Left {
                                    Self::right_rotate(sibling.clone());
                                } else {
                                    Self::left_rotate(sibling.clone());
                                }
                                Self::set_red(sibling.clone());
                                Self::set_black(option_close.clone().unwrap());
                                Self::delete_maintain_rb(node.clone());
                            }
                            // delete case 6: sibling is black, distant is red
                            else if Self::get_color(option_distant.clone()) == NodeColor::Red {
                                // RotateDirRoot(T,P,dir);  P may be the root
                                if direction == NodeDirection::Left {
                                    Self::left_rotate(parent.clone());
                                } else {
                                    Self::right_rotate(parent.clone());
                                }
                                sibling.borrow_mut().color = parent.borrow().color.clone();
                                Self::set_black(parent.clone());
                                Self::set_black(option_distant.clone().unwrap());
                            }
                        }
                    }
                }
            }
        }
    }

    // left and right rotate
    // node is the root of the subtree

    /// Rotate the subtree rooted at this node to the left and
    /// return the new root to this subtree.
    fn left_rotate(node: RBTreeNode<T>) {
        let parent = node.borrow().parent.clone();
        let right = node.borrow().right.clone();

        // println!(
        //     "left ro start {} {:#?}",
        //     node.clone().borrow().value,
        //     node.clone().borrow().color
        // );

        node.borrow_mut().right = right.clone().unwrap().borrow().left.clone();
        if node.borrow().right.is_some() {
            let right = node.borrow().right.clone().unwrap();
            right.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = right.clone();
        right.clone().unwrap().borrow_mut().left = Some(node.clone());
        if parent.is_some() {
            let left = parent.clone().unwrap().borrow().left.clone();
            match left {
                Some(left) if Rc::ptr_eq(&left, &node) => {
                    parent.clone().unwrap().borrow_mut().left = right.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().right = right.clone(),
            }
        }

        right.clone().unwrap().borrow_mut().parent = parent;
    }

    /// Rotate the subtree rooted at this node to the right and
    /// returns the new root to this subtree.
    fn right_rotate(node: RBTreeNode<T>) {
        let parent = node.borrow().parent.clone();
        let left = node.borrow().left.clone();

        // println!(
        //     "right ro start {} {:#?}",
        //     node.clone().borrow().value,
        //     node.clone().borrow().color
        // );
        node.borrow_mut().left = left.clone().unwrap().borrow().right.clone();
        if node.borrow().left.is_some() {
            let left = node.borrow().left.clone().unwrap();
            left.borrow_mut().parent = Some(node.clone());
        }
        node.borrow_mut().parent = left.clone();
        left.clone().unwrap().borrow_mut().right = Some(node.clone());
        if parent.is_some() {
            let right = parent.clone().unwrap().borrow().right.clone();
            match right {
                Some(right) if Rc::ptr_eq(&right, &node) => {
                    parent.clone().unwrap().borrow_mut().right = left.clone();
                }
                _ => parent.clone().unwrap().borrow_mut().left = left.clone(),
            }
        }

        left.clone().unwrap().borrow_mut().parent = parent;
    }

    // Helper function for maintaining
    fn is_left(node: RBTreeNode<T>) -> bool {
        // Return true if the node is the left child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().left.clone() {
                Some(left) => Rc::ptr_eq(&left, &node),
                None => false,
            },
            _ => false,
        }
    }

    // Helper function for maintaining
    fn is_right(node: RBTreeNode<T>) -> bool {
        // Return true if the node is the right child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().right.clone() {
                Some(right) => Rc::ptr_eq(&right, &node),
                None => false,
            },
            _ => false,
        }
    }

    // Helper function for maintaining
    // set node color
    fn set_red(node: RBTreeNode<T>) -> RBTreeNode<T> {
        node.borrow_mut().color = NodeColor::Red;
        return node;
    }

    // Helper function for maintaining
    fn set_black(node: RBTreeNode<T>) -> RBTreeNode<T> {
        node.borrow_mut().color = NodeColor::Black;
        return node;
    }

    // Helper function for maintaining
    fn reverse_color(node: RBTreeNode<T>) {
        if node.borrow().color == NodeColor::Red {
            node.borrow_mut().color = NodeColor::Black;
        } else {
            node.borrow_mut().color = NodeColor::Red;
        }
    }

    fn get_root(node: RBTreeNode<T>) -> OptionRBTreeNode<T> {
        let parent = node.borrow().parent.clone();
        match parent {
            Some(p) => Self::get_root(p),
            None => Some(node),
        }
    }

    // Helper function for maintaining
    // get uncle
    fn get_uncle(node: RBTreeNode<T>) -> OptionRBTreeNode<T> {
        let parent = node.borrow().parent.clone();
        match parent {
            // self is root
            None => None,
            Some(parent) => {
                let grand_parent = parent.borrow().parent.clone();
                match grand_parent {
                    // parent is root
                    None => None,
                    Some(grand_parent) => {
                        if Self::is_left(parent) {
                            grand_parent.borrow().right.clone()
                        } else {
                            grand_parent.borrow().left.clone()
                        }
                    }
                }
            }
        }
    }

    // Helper function for maintaining
    fn get_sibling(node: RBTreeNode<T>) -> OptionRBTreeNode<T> {
        // Get the current node's sibling, or None if it does not exist.
        match node.borrow().parent.clone() {
            None => None,
            Some(parent) => {
                if Self::is_left(node.clone()) {
                    parent.borrow().right.clone()
                } else {
                    parent.borrow().left.clone()
                }
            }
        }
    }

    // Helper function for maintaining
    // make None to be real leaves with black color
    fn get_color(node: OptionRBTreeNode<T>) -> NodeColor {
        match node {
            None => NodeColor::Black,
            Some(node) => node.borrow().color.clone(),
        }
    }

    // Helper function for RBTree::is_valid_red_black_tree
    fn calculate_black_height(node: OptionRBTreeNode<T>) -> Option<usize> {
        match node {
            None => Some(1),
            Some(node) => {
                let left_height = Self::calculate_black_height(node.borrow().left.clone());
                let right_height = Self::calculate_black_height(node.borrow().right.clone());
                match (left_height, right_height) {
                    (Some(left_height), Some(right_height)) => {
                        if left_height != right_height {
                            //The 2 children have unequal depths
                            None
                        } else {
                            let node_color = &node.borrow().color;
                            //Return the black depth of children,plus 1 if the node is black
                            match node_color {
                                NodeColor::Red => Some(left_height),
                                NodeColor::Black => Some(left_height + 1),
                            }
                        }
                    }
                    _ => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotation() {
        let mut tree = RBTree::new();
        tree.insert(30);
        {
            let root = tree.root.clone().unwrap();
            root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                20,
                Some(root.clone()),
            ))));
            root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                40,
                Some(root.clone()),
            ))));

            let left = root.borrow().left.clone().unwrap();
            left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                10,
                Some(left.clone()),
            ))));
            left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                25,
                Some(left.clone()),
            ))));

            let right = root.borrow().right.clone().unwrap();
            right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                35,
                Some(right.clone()),
            ))));
            right.borrow_mut().right = Some(Rc::new(RefCell::new(
                TreeNode::new_black_with_parent(50, Some(right.clone())),
            )));
        }
        let mut after_left_rot = RBTree::new();
        after_left_rot.insert(40);
        {
            let root = after_left_rot.root.clone().unwrap();
            root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                30,
                Some(root.clone()),
            ))));

            let left = root.borrow().left.clone().unwrap();
            left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                20,
                Some(left.clone()),
            ))));
            left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                35,
                Some(left.clone()),
            ))));

            let left = left.borrow().left.clone().unwrap();
            left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                10,
                Some(left.clone()),
            ))));
            left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                25,
                Some(root.clone()),
            ))));

            root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new_black_with_parent(
                50,
                Some(root.clone()),
            ))));
        }
        {
            let root = tree.root.clone().unwrap();
            TreeNode::left_rotate(root);
        }
        let mut tree_container = vec![];
        let mut left_rotate_container = vec![];
        tree.root = TreeNode::get_root(tree.root.clone().unwrap());
        tree.pre_order_traversal_for_test(&mut tree_container);
        after_left_rot.pre_order_traversal_for_test(&mut left_rotate_container);

        assert_eq!(tree_container, left_rotate_container);
    }

    #[test]
    fn tree_traversal() {
        // Test the three different tree traversal functions.
        let mut tree = RBTree::new();
        tree.insert(0);
        vec![16, 16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });
        let mut container = vec![];
        tree.pre_order_traversal_for_test(&mut container);
        assert_eq!(container, vec![8, 0, 20, 16, 24, 22]);
        // let mut container = vec![];
        // RedBlackTreeNode::debug_preorder_traverse(root.clone(), &mut container);
        // assert_eq!(container, vec![0, -16, 16, 8, 22, 20, 24]);

        // let mut container = vec![];
        // RedBlackTreeNode::postorder_traverse(root, &mut container);
        // assert_eq!(container, vec![-16, 8, 20, 24, 22, 16, 0]);
    }

    #[test]
    fn test_insert() {
        let mut rb_tree = RBTree::new();
        rb_tree.insert(12);
        rb_tree.insert(1);
        rb_tree.insert(9);
        rb_tree.insert(2);
        rb_tree.insert(0);
        rb_tree.insert(11);
        rb_tree.insert(7);
        rb_tree.insert(19);
        rb_tree.insert(4);
        rb_tree.insert(15);
        rb_tree.insert(18);
        rb_tree.insert(5);
        rb_tree.insert(14);
        rb_tree.insert(13);
        rb_tree.insert(10);
        rb_tree.insert(16);
        rb_tree.insert(6);
        rb_tree.insert(3);
        rb_tree.insert(8);
        rb_tree.insert(17);

        let result = RBTree::is_valid_red_black_tree(rb_tree.root);
        assert_eq!(result, true);
    }

    #[test]
    fn test_delete() {
        // Test the three different tree traversal functions.
        let mut tree = RBTree::new();
        tree.insert(0);
        vec![16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });

        let root = tree.root.clone().unwrap();
        tree.delete(16);
        let mut container = vec![];
        tree.debug_preorder_traverse(root.clone(), &mut container);
        let result = RBTree::is_valid_red_black_tree(tree.root);
        assert_eq!(result, true);
        //  assert_eq!(container, vec![8, 0, 20, 24, 22]);
    }
}

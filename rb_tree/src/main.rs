use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

type RBTreeNode = Rc<RefCell<TreeNode<u32>>>;
type OptionRBTreeNode = Option<RBTreeNode>;

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub value: T,
    pub parent: OptionRBTreeNode,
    left: OptionRBTreeNode,
    right: OptionRBTreeNode,
    pub p_value: T,
}

impl TreeNode<u32> {
    fn new(value: u32) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: None,
            left: None,
            right: None,
            p_value: 0,
        }
    }
    fn new_with_parent(value: u32, parent: OptionRBTreeNode) -> Self {
        TreeNode {
            color: NodeColor::Red,
            value: value,
            parent: parent,
            left: None,
            right: None,
            p_value: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RBTree {
    root: OptionRBTreeNode,
    // length: u32,
}

impl RBTree {
    fn new() -> Self {
        RBTree {
            root: None,
            // length: 0,
        }
    }

    fn insert(&mut self, insert_value: u32) {
        // self.length += 1;
        let root = self.root.take();
        if root == None {
            self.root = Some(self.set_black(Rc::new(RefCell::new(TreeNode::new(insert_value)))));
        } else {
            self.root = self.node_insert(root.unwrap(), insert_value);
        }
    }

    // TODO: panic user if node is not root
    fn node_insert(&mut self, node: RBTreeNode, insert_value: u32) -> OptionRBTreeNode {
        if node.borrow().value == insert_value {
            return Some(node);
        } else if node.borrow().value > insert_value {
            let left = node.borrow().left.clone();
            match left {
                Some(left) => {
                    self.node_insert(left, insert_value);
                }
                None => {
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(
                        TreeNode::new_with_parent(insert_value, Some(node.clone())),
                    )));
                    let left = node.borrow().left.clone();
                    self.insert_maintain_rb(left.unwrap());
                }
            }
        } else {
            let right = node.borrow().right.clone();
            match right {
                Some(right) => {
                    self.node_insert(right, insert_value);
                }
                None => {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(
                        TreeNode::new_with_parent(insert_value, Some(node.clone())),
                    )));
                    let right = node.borrow().right.clone();
                    self.insert_maintain_rb(right.unwrap());
                }
            }
        }

        // return the root
        let parent = node.borrow().parent.clone();
        match parent {
            Some(parent) => Some(parent),
            None => Some(node),
        }
    }

    fn insert_maintain_rb(&self, node: RBTreeNode) {
        let parent = node.borrow().parent.clone();

        match parent {
            None => {
                // insert case 3: node is root, no parent
                println!("insert case 3: node is root, no parent");
                self.set_black(node);
            }
            // Check parent color
            Some(parent) => {
                println!(
                    "=====start at insert_maintain_rb {:#?} {:#?}",
                    node.borrow().value,
                    parent.clone().borrow().value
                );
                if parent.borrow().color == NodeColor::Black {
                    // insert case 1: parent is black, do not need maintain
                    println!("insert case 1: parent is black, do not need maintain");
                    ();
                }
                // parent is red, need maintain
                else {
                    let grand_parent = parent.borrow().parent.clone();
                    match grand_parent {
                        None => {
                            // insert case 4: grandparent is None, then parent goes to black
                            println!("insert case 4");
                            self.set_black(parent);
                        }
                        Some(grand_parent) => {
                            println!(
                                "===== case 2 or 5 or 6 else parent at insert_maintain_rb {:#?} {:#?} {:#?}",
                                node.borrow().value,
                                parent.clone().borrow().value,
                                grand_parent.clone().borrow().value
                            );
                            // let grand_parent_borrowed = grand_parent.borrow();
                            if grand_parent.borrow().color == NodeColor::Red {
                                panic!("Red violation!");
                            }
                            if self.is_left(parent.clone()) && self.is_left(node.clone()) {
                                let uncle = grand_parent.borrow().right.clone();
                                match uncle {
                                    None => {
                                        // insert case 6.1: left left && uncle is None
                                        println!("insert case 6.1: left left && uncle is None");
                                        self.right_rotate(grand_parent.clone());
                                        let parent = node.borrow().parent.clone();
                                        let parent = parent.unwrap();
                                        self.set_black(parent.clone());
                                        let right = parent.borrow().right.clone().unwrap();
                                        self.set_red(right.clone());
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 161 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            self.set_black(parent);
                                            self.set_black(uncle);
                                            self.set_red(grand_parent.clone());
                                            self.insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 6.1: left left && uncle is black
                                            println!(
                                                "insert case 6.1: left left && uncle is black"
                                            );
                                            self.right_rotate(grand_parent.clone());
                                            let parent = node.borrow().parent.clone();
                                            let parent = parent.unwrap();
                                            self.set_black(parent.clone());
                                            let right = parent.borrow().right.clone().unwrap();
                                            self.set_red(right.clone());
                                        }
                                    }
                                }
                            } else if self.is_right(parent.clone()) && self.is_right(node.clone()) {
                                let uncle = grand_parent.borrow().left.clone();
                                println!(
                                    "===== else self.is_right(parent.clone()) && self.is_right(node.clone() {:#?} {:#?} {:#?}",
                                    node.borrow().value,
                                    parent.clone().borrow().value,
                                    grand_parent.clone().borrow().value
                                );
                                match uncle {
                                    None => {
                                        // insert case 6.2: right right && uncle is None
                                        println!("insert case 6.2: right right && uncle is None");
                                        self.left_rotate(grand_parent.clone());
                                        let parent = node.borrow().parent.clone();
                                        let parent = parent.unwrap();
                                        self.set_black(parent.clone());
                                        let left = parent.borrow().left.clone().unwrap();
                                        self.set_red(left.clone());
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 197 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            self.set_black(parent);
                                            self.set_black(uncle);
                                            self.set_red(grand_parent.clone());
                                            self.insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 6.2: right right && uncle is black
                                            println!(
                                                "insert case 6.2: right right && uncle is black"
                                            );
                                            self.left_rotate(grand_parent.clone());
                                            let parent = node.borrow().parent.clone();
                                            let parent = parent.unwrap();
                                            self.set_black(parent.clone());
                                            let left = parent.borrow().left.clone().unwrap();
                                            self.set_red(left.clone());
                                        }
                                    }
                                }
                            } else if self.is_left(parent.clone()) && self.is_right(node.clone()) {
                                let uncle = grand_parent.borrow().right.clone();
                                match uncle {
                                    None => {
                                        // insert case 5.1: left right  && uncle is None
                                        println!("case 5.1: left right  && uncle is None");
                                        println!(
                                            "===== case 5.1: {:#?} {:#?} {:#?}",
                                            node.borrow().value,
                                            parent.clone().borrow().value,
                                            grand_parent.clone().borrow().value
                                        );
                                        self.left_rotate(parent.clone());
                                        let left_child = node.borrow().left.clone().unwrap();
                                        self.insert_maintain_rb(left_child);
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 237 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            self.set_black(parent);
                                            self.set_black(uncle);
                                            self.set_red(grand_parent.clone());
                                            self.insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 5.1: left right  && uncle is black
                                            println!(
                                                "insert case 5.1: left right  && uncle is black"
                                            );
                                            self.left_rotate(parent.clone());
                                            let left_child = node.borrow().left.clone().unwrap();
                                            self.insert_maintain_rb(left_child);
                                        }
                                    }
                                }
                            } else if self.is_right(parent.clone()) && self.is_left(node.clone()) {
                                let uncle = grand_parent.borrow().left.clone();
                                println!(
                                    "===== else if self.is_right(parent.clone()) && self.is_left(node.clone() {:#?} {:#?} {:#?}",
                                    node.borrow().value,
                                    parent.clone().borrow().value,
                                    grand_parent.borrow().value
                                );
                                match uncle {
                                    None => {
                                        // insert case 5.2: right left && uncle is None
                                        println!("insert case 5.2: right left && uncle is None");
                                        self.right_rotate(parent.clone());
                                        let right_child = node.borrow().right.clone().unwrap();
                                        self.insert_maintain_rb(right_child);
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 282 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            self.set_black(parent);
                                            self.set_black(uncle);
                                            self.set_red(grand_parent.clone());
                                            self.insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 5.2: right left && uncle is black
                                            println!(
                                                "insert case 5.2: right left && uncle is black"
                                            );
                                            self.right_rotate(parent.clone());
                                            let right_child = node.borrow().right.clone().unwrap();
                                            self.insert_maintain_rb(right_child);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // left and right rotate
    // node is the root of the subtree
    fn left_rotate(&self, node: RBTreeNode) {
        let parent = node.borrow().parent.clone();
        let right = node.borrow().right.clone();

        println!(
            "left ro start {} {:#?}",
            node.clone().borrow().value,
            node.clone().borrow().color
        );

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
        // println!("left ro end {:#?}", &self);
        // right
    }

    fn right_rotate(&self, node: RBTreeNode) {
        let parent = node.borrow().parent.clone();
        let left = node.borrow().left.clone();

        println!(
            "right ro start {} {:#?}",
            node.clone().borrow().value,
            node.clone().borrow().color
        );
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
        // println!("right ro end {:#?}", &self);
        // left
    }

    // get uncle
    fn get_uncle(&self, node: RBTreeNode) -> OptionRBTreeNode {
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
                        if self.is_right(parent) {
                            grand_parent.borrow().left.clone()
                        } else {
                            grand_parent.borrow().right.clone()
                        }
                    }
                }
            }
        }
    }

    fn is_left(&self, node: RBTreeNode) -> bool {
        // Return true if the node is the left child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().left.clone() {
                Some(left) => Rc::ptr_eq(&left, &node),
                None => false,
            },
            _ => false,
        }
    }

    fn is_right(&self, node: RBTreeNode) -> bool {
        // Return true if the node is the right child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().right.clone() {
                Some(right) => Rc::ptr_eq(&right, &node),
                None => false,
            },
            _ => false,
        }
    }

    // set node color
    fn set_red(&self, node: RBTreeNode) -> RBTreeNode {
        node.borrow_mut().color = NodeColor::Red;
        return node;
    }

    fn set_black(&self, node: RBTreeNode) -> RBTreeNode {
        node.borrow_mut().color = NodeColor::Black;
        return node;
    }

    fn reverse_color(&self, node: RBTreeNode) {
        if node.borrow().color == NodeColor::Red {
            node.borrow_mut().color = NodeColor::Black;
        } else {
            node.borrow_mut().color = NodeColor::Red;
        }
    }

    // find the min value in its children
    fn get_min_value_in_children(&self, node: RBTreeNode) -> u32 {
        match node.borrow().left.clone() {
            Some(left) => self.get_min_value_in_children(left),
            None => node.borrow().value.clone(),
        }
    }

    // find the max value in its children
    fn get_max_value_in_children(&self, node: RBTreeNode) -> u32 {
        match node.borrow().right.clone() {
            Some(right) => self.get_max_value_in_children(right),
            None => node.borrow().value.clone(),
        }
    }

    fn delete(&mut self, delete_value: u32) {
        let root = self.root.take();
        let result = self.node_delete(root, delete_value);
        self.root = result;
    }

    fn node_delete(&mut self, node: OptionRBTreeNode, delete_value: u32) -> OptionRBTreeNode {
        match node {
            None => None,
            Some(node) => {
                if node.borrow().value == delete_value {
                    let left = node.borrow_mut().left.clone();
                    let right = node.borrow_mut().right.clone();
                    // 1. Two children case: current node has two children
                    // if current node has two children, then recursively replace it with the min value of right
                    // delete the min value of right in the right tree
                    // the goal is to make the problem to be the case where current node has only one child
                    if left.is_some() && right.is_some() {
                        let min_of_right = self.get_min_value_in_children(right.clone().unwrap());
                        node.borrow_mut().value = min_of_right;
                        self.node_delete(right, min_of_right);
                    }
                    // current node has one child or no child
                    else {
                        // 2. Red case: current node is red
                        // it means that current node has no child, just delete this node
                        if node.borrow().color == NodeColor::Red {
                            let parent = node.borrow().parent.clone().unwrap();
                            if self.is_left(node.clone()) {
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
                                // match node.borrow().parent.clone() {
                                //     None => {
                                //         left.borrow_mut().color = NodeColor::Black;
                                //         left.borrow_mut().parent = None;
                                //         return Some(left);
                                //     }
                                //     Some(parent) => {
                                //         if self.is_left(node.clone()) {
                                //             parent.borrow_mut().left = Some(left.clone());
                                //             left.borrow_mut().parent = Some(parent.clone());
                                //         } else {
                                //             parent.borrow_mut().right = Some(left.clone());
                                //             left.borrow_mut().parent = Some(parent.clone());
                                //         }
                                //     }
                                // }

                                node.borrow_mut().value = left.borrow().value;
                                node.borrow_mut().left = left.borrow().left.clone();
                                node.borrow_mut().right = left.borrow().right.clone();
                                if node.borrow().left.is_some() {
                                    let left = node.borrow().left.clone().unwrap();
                                    left.borrow_mut().parent = Some(node.clone());
                                }
                                if node.borrow().right.is_some() {
                                    let right = node.borrow().right.clone().unwrap();
                                    right.borrow_mut().parent = Some(node.clone());
                                }
                            }
                            // 3.2 Black + right red case: current node is black and right child is red, left child is None
                            // delete the current black node and move the right child to the current node place
                            else if left.is_none() && right.is_some() {
                                let right = right.unwrap();
                                // match node.borrow().parent.clone() {
                                //     None => {
                                //         right.borrow_mut().color = NodeColor::Black;
                                //         right.borrow_mut().parent = None;
                                //         return Some(right);
                                //     }
                                //     Some(parent) => {
                                //         if self.is_left(node.clone()) {
                                //             parent.borrow_mut().left = Some(right.clone());
                                //             right.borrow_mut().parent = Some(parent.clone());
                                //         } else {
                                //             parent.borrow_mut().right = Some(right.clone());
                                //             right.borrow_mut().parent = Some(parent.clone());
                                //         }
                                //     }
                                // }
                                node.borrow_mut().value = right.borrow().value;
                                node.borrow_mut().left = right.borrow().left.clone();
                                node.borrow_mut().right = right.borrow().right.clone();
                                if node.borrow().left.is_some() {
                                    let left = node.borrow().left.clone().unwrap();
                                    left.borrow_mut().parent = Some(node.clone());
                                }
                                if node.borrow().right.is_some() {
                                    let right = node.borrow().right.clone().unwrap();
                                    right.borrow_mut().parent = Some(node.clone());
                                }
                            }
                            // 4. Black + no children case: current node is black and has no children
                            else {
                                if node.borrow().parent.clone().is_none() {
                                    // 4.1 current node is the root, then return None
                                    return None;
                                } else {
                                    // 4.2 current node has parent, then call delete_maintain_rb
                                    // and delete the link between current node and its parent
                                    self.delete_maintain_rb(node.clone());
                                    let parent = node.borrow().parent.clone().unwrap();
                                    if self.is_left(node.clone()) {
                                        parent.borrow_mut().left = None;
                                    } else {
                                        parent.borrow_mut().right = None;
                                    }
                                    node.borrow_mut().parent = None;
                                }
                            }
                        }
                    }
                } else if node.borrow().value > delete_value {
                    let left_child = node.borrow().left.clone();
                    if left_child.is_some() {
                        self.node_delete(left_child, delete_value);
                    }
                } else {
                    let right_child = node.borrow_mut().right.take();
                    if right_child.is_some() {
                        self.node_delete(right_child, delete_value);
                    }
                }

                // return the root
                let parent = node.borrow().parent.clone();
                match parent {
                    Some(parent) => Some(parent),
                    None => Some(node),
                }
            }
        }
    }

    fn delete_maintain_rb(&self, node: RBTreeNode) {}

    fn get_sibling(&self, node: RBTreeNode) -> OptionRBTreeNode {
        // Get the current node's sibling, or None if it does not exist.
        match node.borrow().parent.clone() {
            None => None,
            Some(parent) => {
                if self.is_left(node) {
                    parent.borrow().right.clone()
                } else {
                    parent.borrow().left.clone()
                }
            }
        }
    }

    // fn count_leaves(&self) -> u32 {}

    // fn height(&self) -> u32 {}

    // fn in_order_traversal(&self) {}

    // fn is_tree_empty(&self) -> bool {}

    fn preorder_traverse(&self, node: RBTreeNode, container: &mut Vec<u32>) {
        container.push(node.borrow().value);
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.preorder_traverse(left.unwrap(), container);
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.preorder_traverse(right.unwrap(), container);
        }
    }

    fn preorder_traverse_reconstruct(&self, node: RBTreeNode) {
        let temp = match node.borrow().parent.clone() {
            Some(p) => p.borrow().value,
            None => 0,
        };
        node.borrow_mut().p_value = temp;
        node.borrow_mut().parent = None;
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.preorder_traverse_reconstruct(left.unwrap());
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.preorder_traverse_reconstruct(right.unwrap());
        }
    }
}

fn main() {
    let mut rb_tree = RBTree::new();
    // rb_tree.insert(3);
    // let temp = rb_tree.clone();
    // temp.preorder_traverse_reconstruct(rb_tree.root.clone().unwrap());
    // println!("{:#?}", temp);
    // rb_tree.insert(2);
    // let temp = rb_tree.clone();
    // temp.preorder_traverse_reconstruct(rb_tree.root.clone().unwrap());
    // println!("{:#?}", temp);
    // rb_tree.insert(1);
    // rb_tree.insert(4);
    // rb_tree.insert(5);

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
    let temp = rb_tree.clone();
    temp.preorder_traverse_reconstruct(rb_tree.root.clone().unwrap());
    println!("{:#?}", temp);
    // rb_tree.insert(8);
    // rb_tree.insert(11);
    // rb_tree.insert(13);

    // rb_tree.insert(4);
    // rb_tree.insert(5);
    // rb_tree.insert(6);
    // rb_tree.insert(7);
    // println!("{:#?}", rb_tree);
    // rb_tree.insert(6);
    // println!("{:#?}", rb_tree);
    // rb_tree.insert(4);
    // println!("{:#?}", rb_tree);
    // rb_tree.insert(9);
    // rb_tree.delete(5);
    // println!("{:#?}", rb_tree.root);
}

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

        let parent = node.borrow().parent.clone();
        if parent.is_some() {
            parent
        } else {
            Some(node)
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
                            let grand_parent_borrowed = grand_parent.borrow();
                            if grand_parent_borrowed.color == NodeColor::Red {
                                panic!("Red violation!");
                            }
                            if self.is_left(parent.clone()) && self.is_left(node.clone()) {
                                let uncle = grand_parent_borrowed.right.clone();
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
                                let uncle = grand_parent_borrowed.left.clone();
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
                                let uncle = grand_parent_borrowed.right.clone();
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
                                        self.insert_maintain_rb(
                                            node.borrow().left.clone().unwrap(),
                                        );
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
                                            self.insert_maintain_rb(
                                                node.borrow().left.clone().unwrap(),
                                            );
                                        }
                                    }
                                }
                            } else if self.is_right(parent.clone()) && self.is_left(node.clone()) {
                                let uncle = grand_parent_borrowed.right.clone();
                                println!(
                                    "===== else if self.is_right(parent.clone()) && self.is_left(node.clone() {:#?} {:#?} {:#?}",
                                    node.borrow().value,
                                    parent.clone().borrow().value,
                                    grand_parent_borrowed.value
                                );
                                match uncle {
                                    None => {
                                        // insert case 5.2: right left && uncle is None
                                        println!("insert case 5.2: right left && uncle is None");
                                        self.right_rotate(parent.clone());
                                        self.insert_maintain_rb(
                                            node.borrow().right.clone().unwrap(),
                                        );
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
                                            self.insert_maintain_rb(
                                                node.borrow().right.clone().unwrap(),
                                            );
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

    // find the min node
    fn find_min(&self, node: &OptionRBTreeNode) -> Option<u32> {
        match node {
            None => None,
            Some(inner) => {
                if inner.borrow().left.is_some() {
                    self.find_min(&inner.borrow().left)
                } else {
                    Some(inner.borrow().value.clone())
                }
            }
        }
    }

    fn delete(&mut self, value: u32) {
        let root = self.root.take();
        let result = self.node_delete(root, value);
        self.root = result;
    }

    fn node_delete(&mut self, node: OptionRBTreeNode, value: u32) -> OptionRBTreeNode {
        match node {
            None => None,
            Some(inner) => {
                if inner.borrow_mut().value == value {
                    let left = inner.borrow_mut().left.take();
                    let right = inner.borrow_mut().right.take();
                    if left.is_some() && right.is_some() {
                        let min_of_right = self.find_min(&right);
                        inner.borrow_mut().left = left;
                        if let Some(tmp_value) = min_of_right {
                            inner.borrow_mut().value = tmp_value;
                            let result = self.node_delete(right, tmp_value);
                            inner.borrow_mut().right = result;
                            Some(inner)
                        } else {
                            None
                        }
                    } else if left.is_some() {
                        left
                    } else if right.is_some() {
                        right
                    } else {
                        None
                    }
                } else if inner.borrow_mut().value > value {
                    let left = inner.borrow_mut().left.take();
                    let result = self.node_delete(left, value);
                    inner.borrow_mut().left = result;
                    Some(inner)
                } else {
                    let right = inner.borrow_mut().right.take();
                    let result = self.node_delete(right, value);
                    inner.borrow_mut().right = result;
                    Some(inner)
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

    rb_tree.insert(15);
    rb_tree.insert(17);
    rb_tree.insert(22);
    rb_tree.insert(25);
    rb_tree.insert(27);
    rb_tree.insert(1);
    rb_tree.insert(6);
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

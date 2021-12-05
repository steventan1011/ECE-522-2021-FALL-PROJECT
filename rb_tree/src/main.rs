use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
enum NodeDirection {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
struct RBTree {
    root: OptionRBTreeNode,
}

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub value: T,
    pub parent: OptionRBTreeNode,
    left: OptionRBTreeNode,
    right: OptionRBTreeNode,
    pub p_value: T,
}

type RBTreeNode = Rc<RefCell<TreeNode<u32>>>;
type OptionRBTreeNode = Option<RBTreeNode>;

impl NodeColor {
    fn to_string(&self) -> &str {
        if self == &NodeColor::Red {
            "r"
        } else {
            "b"
        }
    }
}

// RBTree
impl RBTree {
    fn new() -> Self {
        RBTree { root: None }
    }

    fn insert(&mut self, insert_value: u32) {
        let root = self.root.clone();
        match root {
            None => {
                self.root = Some(TreeNode::set_black(Rc::new(RefCell::new(TreeNode::new(
                    insert_value,
                )))))
            }
            Some(root) => self.root = TreeNode::node_insert(root, insert_value),
        }
    }

    fn delete(&mut self, delete_value: u32) {
        let root = self.root.clone();
        let result = TreeNode::node_delete(root, delete_value);
        self.root = result;
    }

    // fn count_leaves(&self) -> u32 {}

    // fn height(&self) -> u32 {}

    fn in_order_traversal(&self) {
        match self.root.clone() {
            None => return,
            Some(root) => TreeNode::in_order_traversal(root),
        }
    }

    // fn is_tree_empty(&self) -> bool {}

    fn preorder_traverse(&self, node: RBTreeNode, container: &mut Vec<String>) {
        container.push(node.borrow().value.to_string() + node.borrow().color.to_string());
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

    fn inorder_traverse(&self, node: RBTreeNode, container: &mut Vec<String>) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.inorder_traverse(left.unwrap(), container);
        }
        container.push(node.borrow().value.to_string() + node.borrow().color.to_string());
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.inorder_traverse(right.unwrap(), container);
        }
    }
}

// TreeNode
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

    fn node_insert(node: RBTreeNode, insert_value: u32) -> OptionRBTreeNode {
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

    fn insert_maintain_rb(node: RBTreeNode) {
        let parent = node.borrow().parent.clone();

        match parent {
            None => {
                // insert case 3: node is root, no parent
                println!("insert case 3: node is root, no parent");
                Self::set_black(node);
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
                            Self::set_black(parent);
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
                            if Self::is_left(parent.clone()) && Self::is_left(node.clone()) {
                                let uncle = grand_parent.borrow().right.clone();
                                match uncle {
                                    None => {
                                        // insert case 6.1: left left && uncle is None
                                        println!("insert case 6.1: left left && uncle is None");
                                        Self::right_rotate(grand_parent.clone());
                                        let parent = node.borrow().parent.clone().unwrap();
                                        Self::set_black(parent.clone());
                                        let right = parent.borrow().right.clone().unwrap();
                                        Self::set_red(right.clone());
                                    }
                                    Some(uncle) => {
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
                                        } else {
                                            // insert case 6.1: left left && uncle is black
                                            println!(
                                                "insert case 6.1: left left && uncle is black"
                                            );
                                            Self::right_rotate(grand_parent.clone());
                                            let parent = node.borrow().parent.clone().unwrap();
                                            Self::set_black(parent.clone());
                                            let right = parent.borrow().right.clone().unwrap();
                                            Self::set_red(right.clone());
                                        }
                                    }
                                }
                            } else if Self::is_right(parent.clone()) && Self::is_right(node.clone())
                            {
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
                                        Self::left_rotate(grand_parent.clone());
                                        let parent = node.borrow().parent.clone().unwrap();
                                        Self::set_black(parent.clone());
                                        let left = parent.borrow().left.clone().unwrap();
                                        Self::set_red(left.clone());
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 197 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            Self::set_black(parent.clone());
                                            Self::set_black(uncle.clone());
                                            Self::set_red(grand_parent.clone());
                                            Self::insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 6.2: right right && uncle is black
                                            println!(
                                                "insert case 6.2: right right && uncle is black"
                                            );
                                            Self::left_rotate(grand_parent.clone());
                                            let parent = node.borrow().parent.clone().unwrap();
                                            Self::set_black(parent.clone());
                                            let left = parent.borrow().left.clone().unwrap();
                                            Self::set_red(left.clone());
                                        }
                                    }
                                }
                            } else if Self::is_left(parent.clone()) && Self::is_right(node.clone())
                            {
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
                                        Self::left_rotate(parent.clone());
                                        let left_child = node.borrow().left.clone().unwrap();
                                        Self::insert_maintain_rb(left_child.clone());
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 237 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            Self::set_black(parent.clone());
                                            Self::set_black(uncle.clone());
                                            Self::set_red(grand_parent.clone());
                                            Self::insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 5.1: left right  && uncle is black
                                            println!(
                                                "insert case 5.1: left right  && uncle is black"
                                            );
                                            Self::left_rotate(parent.clone());
                                            let left_child = node.borrow().left.clone().unwrap();
                                            Self::insert_maintain_rb(left_child.clone());
                                        }
                                    }
                                }
                            } else if Self::is_right(parent.clone()) && Self::is_left(node.clone())
                            {
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
                                        Self::right_rotate(parent.clone());
                                        let right_child = node.borrow().right.clone().unwrap();
                                        Self::insert_maintain_rb(right_child.clone());
                                    }
                                    Some(uncle) => {
                                        println!(
                                            "===== uncle 282 at insert_maintain_rb {:#?}",
                                            uncle.clone().borrow().value,
                                        );
                                        if uncle.borrow().color == NodeColor::Red {
                                            // insert case 2: uncle is red
                                            println!("insert case 2: uncle is red");
                                            Self::set_black(parent.clone());
                                            Self::set_black(uncle.clone());
                                            Self::set_red(grand_parent.clone());
                                            Self::insert_maintain_rb(grand_parent.clone());
                                        } else {
                                            // insert case 5.2: right left && uncle is black
                                            println!(
                                                "insert case 5.2: right left && uncle is black"
                                            );
                                            Self::right_rotate(parent.clone());
                                            let right_child = node.borrow().right.clone().unwrap();
                                            Self::insert_maintain_rb(right_child.clone());
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

    fn node_delete(node: OptionRBTreeNode, delete_value: u32) -> OptionRBTreeNode {
        match node {
            None => None,
            Some(node) => {
                if node.borrow().value > delete_value {
                    let left_child = node.borrow().left.clone();
                    if left_child.is_some() {
                        Self::node_delete(left_child, delete_value);
                    }
                } else if node.borrow().value < delete_value {
                    let right_child = node.borrow_mut().right.clone();
                    if right_child.is_some() {
                        Self::node_delete(right_child, delete_value);
                    }
                } else {
                    let left = node.borrow_mut().left.clone();
                    let right = node.borrow_mut().right.clone();
                    // 1. Two children case: current node has two children
                    // if current node has two children, then recursively replace it with the min value of right
                    // delete the min value of right in the right tree
                    // the goal is to make the problem to be the case where current node has only one child
                    if left.is_some() && right.is_some() {
                        let min_of_right = Self::get_min_value_in_children(right.clone().unwrap());
                        node.borrow_mut().value = min_of_right;
                        Self::node_delete(right, min_of_right);
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
        }
    }

    fn delete_maintain_rb(node: RBTreeNode) {
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
    fn left_rotate(node: RBTreeNode) {
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
    }

    fn right_rotate(node: RBTreeNode) {
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
    }

    fn is_left(node: RBTreeNode) -> bool {
        // Return true if the node is the left child of its parent.
        match node.borrow().parent.clone() {
            Some(parent) => match parent.borrow().left.clone() {
                Some(left) => Rc::ptr_eq(&left, &node),
                None => false,
            },
            _ => false,
        }
    }

    fn is_right(node: RBTreeNode) -> bool {
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
    fn set_red(node: RBTreeNode) -> RBTreeNode {
        node.borrow_mut().color = NodeColor::Red;
        return node;
    }

    fn set_black(node: RBTreeNode) -> RBTreeNode {
        node.borrow_mut().color = NodeColor::Black;
        return node;
    }

    fn reverse_color(node: RBTreeNode) {
        if node.borrow().color == NodeColor::Red {
            node.borrow_mut().color = NodeColor::Black;
        } else {
            node.borrow_mut().color = NodeColor::Red;
        }
    }

    // find the min value in its children
    fn get_min_value_in_children(node: RBTreeNode) -> u32 {
        match node.borrow().left.clone() {
            Some(left) => Self::get_min_value_in_children(left),
            None => node.borrow().value.clone(),
        }
    }

    // find the max value in its children
    fn get_max_value_in_children(node: RBTreeNode) -> u32 {
        match node.borrow().right.clone() {
            Some(right) => Self::get_max_value_in_children(right),
            None => node.borrow().value.clone(),
        }
    }

    fn get_root(node: RBTreeNode) -> OptionRBTreeNode {
        let parent = node.borrow().parent.clone();
        match parent {
            Some(_) => parent,
            None => Some(node),
        }
    }

    // get uncle
    fn get_uncle(node: RBTreeNode) -> OptionRBTreeNode {
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

    fn get_sibling(node: RBTreeNode) -> OptionRBTreeNode {
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

    // make None to be real leaves with black color
    fn get_color(node: OptionRBTreeNode) -> NodeColor {
        match node {
            None => NodeColor::Black,
            Some(node) => node.borrow().color.clone(),
        }
    }

    fn in_order_traversal(node: RBTreeNode) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            Self::in_order_traversal(left.unwrap());
        }
        print!("{:?} ", node.borrow().value);
        let right = node.borrow().right.clone();
        if right.is_some() {
            Self::in_order_traversal(right.unwrap());
        }
    }
}

fn main() {
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
    // delete
    rb_tree.delete(12);
    rb_tree.delete(1);
    rb_tree.delete(9);
    rb_tree.delete(2);
    rb_tree.delete(0);
    rb_tree.delete(11);
    rb_tree.delete(7);
    rb_tree.delete(19);
    rb_tree.delete(4);
    rb_tree.delete(15);
    rb_tree.delete(18);
    rb_tree.delete(5);
    rb_tree.delete(14);
    // rb_tree.delete(13);
    // rb_tree.delete(10);
    // rb_tree.delete(16);
    // rb_tree.delete(6);
    // rb_tree.delete(3);
    // rb_tree.delete(8);
    // rb_tree.delete(17);
    let temp = rb_tree.clone();
    temp.preorder_traverse_reconstruct(rb_tree.root.clone().unwrap());
    println!("{:#?}", temp);
    let container: &mut Vec<String> = &mut vec![];
    rb_tree
        .clone()
        .preorder_traverse(rb_tree.root.clone().unwrap(), container);
    println!("preorder: {:?}", container);
    let container: &mut Vec<String> = &mut vec![];
    rb_tree
        .clone()
        .inorder_traverse(rb_tree.root.clone().unwrap(), container);
    println!("inorder: {:?}", container);

    rb_tree.in_order_traversal();
}

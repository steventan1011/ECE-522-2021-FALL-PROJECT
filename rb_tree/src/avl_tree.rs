use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::{max, Ordering};
use std::io;
use std::fmt;

type AVLTreeNode = Rc<RefCell<TreeNode<u32>>>;
type OptionAVLTreeNode = Option<AVLTreeNode>;

struct TreeNode<T:fmt::Debug> {
    pub value: T,
    left: OptionAVLTreeNode,
    right: OptionAVLTreeNode,
    height: usize,
}

impl TreeNode<u32> {
    // Create a new node of type OptionAVLTreeNode
    fn new(value: u32) -> OptionAVLTreeNode {
        Some(Rc::new(RefCell::new(Self {
            value,
            left: None,
            right: None,
            height: 1, // default height of a new node is 1，which is a leave
        })))
    }
}

struct AVLTree  {
    root: OptionAVLTreeNode,
}

impl AVLTree {
    fn new() -> Self {
        Self {
            root: None,
        }
    }
    fn insert(&mut self, insert_value: u32) {
        let root = self.root.take();
        // TreeNode is type OptionAVLTreeNode, so the code is simplified.
        match root {
            None => self.root = TreeNode::new(insert_value),
            Some(n) => self.root = self.node_insert(Some(n), insert_value),
        }
    }

    fn node_insert(&mut self, node: OptionAVLTreeNode, insert_value: u32) -> OptionAVLTreeNode {
        let temp_node = match node {
            Some(mut n) => {
                let node_value = n.borrow().value;
                if insert_value < node_value  {
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = self.node_insert(left, insert_value);
                } else if insert_value > node_value {
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = self.node_insert(right, insert_value);
                } else {
                    n.borrow_mut().value = insert_value; // equal, update value
                }
                n
            }
            None => TreeNode::new(insert_value).unwrap(),            
        };

        // update height
        temp_node.borrow_mut().height = self.get_left_height(&temp_node)
        .max(self.get_right_height(&temp_node)) + 1;

        // update balance factor
        let mut balance_factor = self.get_balance_factor(&temp_node);
        if balance_factor.abs() > 1.0 {
            println!("unbalanced: {}", balance_factor);
        }

        Some(temp_node)
    }

    fn get_height(&self, node: OptionAVLTreeNode) -> usize {
        // default height of an empty tree is 0
        node.map_or(0, |n| n.borrow().height)
    }
    fn get_left_height(&self, n: &AVLTreeNode) -> usize {
        self.get_height(n.borrow().left.clone())
    }

    fn get_right_height(&self, n: &AVLTreeNode) -> usize {
        self.get_height(n.borrow().right.clone())
    }

    fn get_balance_factor(&self, n: &AVLTreeNode) -> f64 {
        self.get_left_height(n) as f64 - self.get_right_height(n) as f64
    }


    // fn delete(&mut self, value: u32) {}

    // fn count_leaves(&self) -> u32 {}

    // fn height(&self) -> u32 {}

    // fn in_order_traversal(&self) {}

    // fn is_tree_empty(&self) -> bool {}

}

    
// fn main() {
//     let mut avl_tree = AVLTree::new();
//     avl_tree.insert(2);
//     avl_tree.insert(1);
//     avl_tree.insert(3);
//     println!("{:#?}", avl_tree.print_inorder());
// }
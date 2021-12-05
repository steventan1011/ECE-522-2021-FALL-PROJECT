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

        //println!("insert node: {}", temp_node.borrow().value.clone());

        // maintain
        // case LL: right rotate
        if balance_factor > 1.0 && self.get_balance_factor(&temp_node.borrow().left.clone().unwrap()) >= 0.0 {
            return Some(self.right_rotate(temp_node))
        }

        // case RR: left rotate
        if balance_factor < -1.0 && self.get_balance_factor(&temp_node.borrow().right.clone().unwrap()) <= 0.0 {
            return Some(self.left_rotate(temp_node))
        }

        // case LR: left rotate + right rotate
        if balance_factor > 1.0 && self.get_balance_factor(&temp_node.borrow().left.clone().unwrap()) < 0.0 {
            temp_node.borrow_mut().left = Some(self.left_rotate(temp_node.borrow_mut().left.unwrap()));
            return Some(self.right_rotate(temp_node))
        }

        // case RL: right rotate + left rotate
        if balance_factor < -1.0 && self.get_balance_factor(&temp_node.borrow().right.clone().unwrap()) > 0.0 {
            temp_node.borrow_mut().right = Some(self.right_rotate(temp_node.borrow_mut().right.unwrap()));
            return Some(self.left_rotate(temp_node))
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

    fn get_root(&self) -> &OptionAVLTreeNode {
        &self.root
    }

    //Determine whether the tree is balanced
    fn is_balanced(&self) -> bool {
        match self.get_root() {
            Some(node) => self._is_balanced(node), // Incorrect type of parameter
            None => true,
        }
    }

    // Determine whether the tree with node as root is balanced
    fn _is_balanced(&mut self, node: OptionAVLTreeNode) -> bool {
        let mut balance_factor = self.get_balance_factor(&node.unwrap());
        if balance_factor.abs() > 1.0 {
            false
        } else {
            self._is_balanced(node.unwrap().borrow_mut().left.clone()) &&
                self._is_balanced(node.unwrap().borrow_mut().right.clone())
        }
    }

    //                 y                                     x
    //               /    \                                 /   \
    //              x     T4      right rotate (y)         z     y
    //           /   \            ---------------->       / \   / \
    //          z     T3             return x            T1 T2 T3 T4
    //         /   \
    //        T1   T2
    fn right_rotate(&self, y: AVLTreeNode) -> AVLTreeNode {
        let mut x = y.borrow().left.clone().unwrap();
        let mut T3 = x.borrow().right.clone().unwrap();

        // right rotate
        x.borrow_mut().right = Some(y);
        y.borrow_mut().left = Some(T3);

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y)
            .max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x)
            .max(self.get_right_height(&x)) + 1;

        return x
    }

    //                 y                                        x
    //               /    \                                   /   \
    //              T1     x        left rotate (y)          y     z
    //                   /   \      ---------------->       / \   / \
    //                 T2     z        return x            T1 T2 T3 T4
    //                       /  \
    //                      T3   T4
    fn left_rotate(&self, y: AVLTreeNode) -> AVLTreeNode {
        let mut x = y.borrow().right.clone().unwrap();
        let mut T2 = x.borrow().left.clone().unwrap();

        // left rotate
        x.borrow_mut().left = Some(y);
        y.borrow_mut().right = Some(T2);

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y)
            .max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x)
            .max(self.get_right_height(&x)) + 1;

        return x
    }


    // fn delete(&mut self, value: u32) {}

    // fn count_leaves(&self) -> u32 {}

    // fn height(&self) -> u32 {}

    // fn in_order_traversal(&self) {}

    // fn is_tree_empty(&self) -> bool {}

}

fn main() {
    let mut avl_tree = AVLTree::new();
    avl_tree.insert(2);
    avl_tree.insert(1);
    avl_tree.insert(3);
    avl_tree.insert(4);
    avl_tree.insert(5);
    //println!("{:#?}", avl_tree);
}
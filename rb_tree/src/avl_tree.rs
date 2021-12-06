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
    fn get_left(&self) -> &OptionAVLTreeNode { return &self.left; }
    fn get_right(&self) -> &OptionAVLTreeNode { return &self.right; }
    fn get_data(&self) -> u32 { return self.value; }

    fn min(&self) -> u32 {
        self.get_left().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().min()
        )
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
        let ret_node = match node {
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
        ret_node.borrow_mut().height = self.get_left_height(&ret_node)
            .max(self.get_right_height(&ret_node)) + 1;

        // update balance factor
        let mut balance_factor = self.get_balance_factor(&ret_node);
        if balance_factor.abs() > 1.0 {
            println!("unbalanced: {}", balance_factor);
        }

        //println!("insert node: {}", ret_node.borrow().value.clone());

        // maintain
        // case LL: right rotate
        if balance_factor > 1.0 && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) >= 0.0 {
            return Some(self.right_rotate(ret_node))
        }

        // case RR: left rotate
        if balance_factor < -1.0 && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) <= 0.0 {
            return Some(self.left_rotate(ret_node))
        }

        // case LR: left rotate + right rotate
        if balance_factor > 1.0 && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) < 0.0 {
            ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
            return Some(self.right_rotate(ret_node))
        }

        // case RL: right rotate + left rotate
        if balance_factor < -1.0 && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) > 0.0 {
            ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
            return Some(self.left_rotate(ret_node))
        }
        Some(ret_node)
    }

    fn delete(&mut self, delete_value: u32) {
        let root = self.root.take();
        match root {
            None => return, // 这棵树是空的，没办法delete，所以什么都没发生??? 还是应该返回null？？？？？？？
            Some(n) => self.root = self.node_delete(Some(n), delete_value),
        }
    }

    // delete node, return new root
    fn node_delete(&mut self, node: OptionAVLTreeNode, delete_value: u32) -> OptionAVLTreeNode {
        let ret_node = match node {
            None => node.unwrap(), // 遍历到叶子节点，但是还是没有找到，所以应该返回null，还是说因为叶子节点就是null，所以返回node就可以？？？
            Some(mut n) => {
                let node_value = n.borrow().value;
                if delete_value < node_value  { // look left
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = self.node_delete(left, delete_value);
                    n
                }
                else if delete_value > node_value { // look right
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = self.node_delete(right, delete_value);
                    n
                }
                else { // found the node which should be deleted
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();
                    let ret = match (left.clone(), right.clone()) {
                        (None, Some(r)) => r,  // The left subtree of the node to be deleted is empty, r is new root
                        (Some(l), None) => l, // The right subtree of the node to be deleted is empty, l is new root

                        // 我想通过左子树为空/右子树为空/左右都不为空，这三种情况进行分类，但是缺少左右子树都为空
                        // 如果把(None, Some(r)) => r，改成(None, _) => right.unwrap()???，或者改成(_, Some(r)) => r???
                        (None, None) => None.unwrap(),

                        //The left and right subtrees of the node to be deleted(node n) are not empty.
                        // Find the smallest node A that is larger than the node n.
                        (Some(left), Some(right)) => {
                            let min_value = right.borrow().min(); // Find the value of node A which is the minimum value of the right subtree
                            n.borrow_mut().value = min_value; // Change the value of node n to the value of node A.
                            let right = n.borrow().right.clone().take();
                            n.borrow_mut().right = self.node_delete(right, min_value); // Delete the node A in the right subtree.
                            n // return new root
                        },
                    };
                    ret
                }
            },
        };

        // update and maintain
        match Some(ret_node.clone()) {
            None => Some(ret_node),
            Some(n) => {
                // update height
                ret_node.borrow_mut().height = self.get_left_height(&ret_node) // 借用了发生移动的
                    .max(self.get_right_height(&ret_node)) + 1;

                // update balance factor
                let mut balance_factor = self.get_balance_factor(&ret_node);
                if balance_factor.abs() > 1.0 {
                    println!("unbalanced: {}", balance_factor);
                }

                // maintain
                // case LL: right rotate
                if balance_factor > 1.0 && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) >= 0.0 {
                    return Some(self.right_rotate(ret_node))
                }

                // case RR: left rotate
                if balance_factor < -1.0 && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) <= 0.0 {
                    return Some(self.left_rotate(ret_node))
                }

                // case LR: left rotate + right rotate
                if balance_factor > 1.0 && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) < 0.0 {
                    ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
                    return Some(self.right_rotate(ret_node))
                }

                // case RL: right rotate + left rotate
                if balance_factor < -1.0 && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) > 0.0 {
                    ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
                    return Some(self.left_rotate(ret_node))
                }
                Some(ret_node)
            }
        }
    }

    fn min(&self) -> Option<u32> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().min()),
        }
    }
    fn get_root(&self) -> &OptionAVLTreeNode { &self.root }


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

    // //Determine whether the tree is balanced
    // fn is_balanced(&self) -> bool {
    //     match self.get_root() {
    //         Some(node) => self._is_balanced(node), // node类型错误
    //         None => true,
    //     }
    // }

    // // Determine whether the tree with node as root is balanced
    // fn _is_balanced(&mut self, node: OptionAVLTreeNode) -> bool {
    //     let mut balance_factor = self.get_balance_factor(&node.unwrap());
    //     if balance_factor.abs() > 1.0 {
    //         false
    //     } else {
    //         self._is_balanced(node.unwrap().borrow_mut().left.clone()) &&
    //         self._is_balanced(node.unwrap().borrow_mut().right.clone())
    //     }
    // }

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
        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().left = Some(T3); // 借用了发生移动的y

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
        x.borrow_mut().left = Some(y.clone());
        y.borrow_mut().right = Some(T2); // 借用了发生移动的y

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y)
            .max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x)
            .max(self.get_right_height(&x)) + 1;

        return x
    }

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
//     avl_tree.insert(4);
//     avl_tree.insert(5);
//     //println!("{:#?}", avl_tree);
// }
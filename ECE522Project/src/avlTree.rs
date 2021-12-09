//! AVL tree
//!
//! You can generate an AVL tree, and insert or delete nodes.

use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::{max, Ordering};
use std::io;
use std::fmt;

type AVLTreeNode = Rc<RefCell<TreeNode<u32>>>;
type OptionAVLTreeNode = Option<AVLTreeNode>;

/// Node struct for AVLTree
struct TreeNode<T:fmt::Debug> {
    pub value: T,
    left: OptionAVLTreeNode,
    right: OptionAVLTreeNode,
    height: usize,
}

/// Implementations of AVLTreeNode
impl TreeNode<u32> {
    /// Create a new node of type OptionAVLTreeNode , which will be called by [AVLTree](struct.AVLTree.html)
    fn new(value: u32) -> OptionAVLTreeNode {
        Some(Rc::new(RefCell::new(Self {
            value,
            left: None,
            right: None,
            height: 1, // default height of a new node is 1，which is a leave
        })))
    }

    // Helper functions of TreeNode::min
    fn get_left(&self) -> &OptionAVLTreeNode { return &self.left; }
    fn get_right(&self) -> &OptionAVLTreeNode { return &self.right; }
    fn get_data(&self) -> u32 { return self.value; }

    /// Return the minimum value of current node, which will be called by
    /// [AVLTree.min](struct.AVLTree.html#method.min)
    fn min(&self) -> u32 {
        self.get_left().as_ref().map_or(
            self.get_data(),
            |x| x.borrow_mut().min()
        )
    }

    /// Return the leaves number of current node, which will be called by
    /// [AVLTree.count_leaves](struct.AVLTree.html#method.count_leaves)
    fn count_leaves(node: AVLTreeNode) -> u32 {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        if left.is_none() && right.is_none() {
            2
        } else if left.is_none() && right.is_some() {
            Self::count_leaves(right.clone().unwrap())
        } else if left.is_some() && right.is_none() {
            Self::count_leaves(left.clone().unwrap())
        } else {
            Self::count_leaves(left.clone().unwrap()) + Self::count_leaves(right.clone().unwrap())
        }
    }

    /// Return the height of current node, which will be called by
    /// [AVLTree.height](struct.AVLTree.html#method.height)
    // Node有height属性，可以简化？
    fn get_height(node: AVLTreeNode) -> u32 {
        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();
        let left_height = left.map(|l| Self::get_height(l.clone())).unwrap_or(1);
        let right_height = right.map(|r| Self::get_height(r.clone())).unwrap_or(1);
        return max(left_height, right_height) + 1;
    }

    /// Print nodes inorder, which will be called by
    /// [AVLTree.in_order_traversal](struct.AVLTree.html#method.in_order_traversal)
    fn in_order_traversal(node: AVLTreeNode) {
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

    /// Print nodes preorder, which will be called by
    /// [AVLTree.preorder_traversal](struct.AVLTree.html#method.preorder_traversal)
    fn preorder_traversal(node: AVLTreeNode) {
        print!("{:?} ", node.borrow().value);
        let left = node.borrow().left.clone();
        if left.is_some() {
            Self::preorder_traversal(left.unwrap());
        }
        let right = node.borrow().right.clone();
        if right.is_some() {
            Self::preorder_traversal(right.unwrap());
        }
    }
}

/// Structure of AVLTree
pub struct AVLTree  {
    root: OptionAVLTreeNode,
}

/// Implementations of AVLTree
impl AVLTree {

    /// Creates a new AVL tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// ```
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    /// Counts leaves(None nodes) of AVL tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// avl_tree.insert(1);
    /// println!("{}", avl_tree.count_leaves());  // 2
    /// avl_tree.insert(2);
    /// println!("{}", avl_tree.count_leaves());  // 3
    /// avl_tree.insert(3);
    /// println!("{}", avl_tree.count_leaves());  // 4
    ///
    /// let mut leaf_number = avl_tree.count_leaves();
    /// assert_eq!(4, leaf_number);
    /// ```
    pub fn count_leaves(&self) -> u32 {
        match self.root.clone() {
            None => 0,
            Some(node) => TreeNode::count_leaves(node),
        }
    }

    /// Gets height of AVL tree (from root to leaves)
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// assert_eq!(0, avl_tree.height());
    /// avl_tree.insert(1);
    /// assert_eq!(2, avl_tree.height());
    ///avl_tree.insert(2);
    /// assert_eq!(3, avl_tree.height());
    /// ```
    pub fn height(&self) -> u32 {
        match self.root.clone() {
            None => 0,
            Some(node) => TreeNode::get_height(node),
        }
    }

    /// Prints AVL tree inorder
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// avl_tree.insert(1);
    /// avl_tree.insert(3);
    /// avl_tree.insert(5);
    /// avl_tree.insert(2);
    /// avl_tree.insert(4);
    /// avl_tree.in_order_traversal(); // Inorder traversal: 1 2 3 4 5
    ///
    // inorder traverse
    pub fn in_order_traversal(&self) {
        print!("Inorder traversal: ");
        match self.root.clone() {
            None => print!("the tree does not have node"),
            Some(root) => TreeNode::in_order_traversal(root),
        }
        println!()
    }

    /// Prints AVL tree preorder
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// avl_tree.insert(1);
    /// avl_tree.insert(3);
    /// avl_tree.insert(5);
    /// avl_tree.insert(2);
    /// avl_tree.insert(4);
    /// avl_tree.preorder_traversal(); // Preorder traversal: 3 1 2 5 4
    ///
    fn preorder_traversal(&self) {
        print!("Preorder traversal: ");
        match self.root.clone() {
            None => print!("the tree does not have node"),
            Some(root) => TreeNode::preorder_traversal(root),
        }
        println!()
    }


    /// Judge if the AVL tree is empty
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// println!("{}", avl_tree.is_empty());  // true
    /// tree.insert(1);
    /// println!("{}", avl_tree.is_empty());  // false
    /// ```
    pub fn is_tree_empty(&self) -> bool {
        self.root.clone().map(|_| false).unwrap_or(true)
    }


    /// Inserts a new value to AVL tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// avl_tree.insert(1);
    /// ```
    pub fn insert(&mut self, insert_value: u32) {
        let root = self.root.take();
        // TreeNode is type OptionAVLTreeNode, so the code is simplified.
        match root {
            None => self.root = TreeNode::new(insert_value),
            Some(n) => self.root = self.node_insert(Some(n), insert_value),
        }
    }

    /// Inserts a node, return a new root, which will be called by
    /// [AVLTree.insert](struct.AVLTree.html#method.insert)
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
        // if balance_factor.abs() > 1.0 {
        //     println!("unbalanced: {}", balance_factor);
        // }

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
            // ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
            // return Some(self.right_rotate(ret_node))

            let left = ret_node.borrow().left.clone().take().unwrap();
            ret_node.borrow_mut().left = Some(self.left_rotate(left));
            return Some(self.right_rotate(ret_node))
        }

        // case RL: right rotate + left rotate
        if balance_factor < -1.0 && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) > 0.0 {
            // ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
            // return Some(self.left_rotate(ret_node))

            let right = ret_node.borrow().right.clone().take().unwrap();
            ret_node.borrow_mut().right = Some(self.right_rotate(right));
            return Some(self.left_rotate(ret_node))
        }
        Some(ret_node)
    }

    /// Delete a value from AVL tree
    ///
    /// # Example
    ///
    /// ```
    /// let mut avl_tree = AVLTree::new();
    /// avl_tree.insert(1);
    /// avl_tree.delete(1);
    /// ```
    pub fn delete(&mut self, delete_value: u32) {
        let root = self.root.take();
        match root {
            None => return, // 这棵树是空的，没办法delete，所以什么都没发生??? 还是应该返回null？？？？？？？
            Some(n) => self.root = self.node_delete(Some(n), delete_value),
        }
    }
    /// Deletes a node, return a new root, which will be called by
    /// [AVLTree.delete](struct.AVLTree.html#method.delete)
    // delete node, return new root
    fn node_delete(&mut self, node: OptionAVLTreeNode, delete_value: u32) -> OptionAVLTreeNode {
        let ret_node = match node {
            // None => node.unwrap(), // 遍历到叶子节点，但是还是没有找到，所以应该返回null，还是说因为叶子节点就是null，所以返回node就可以？？？
            // Some(mut n) => {
            //     let node_value = n.borrow().value;
            //     if delete_value < node_value  { // look left
            //         let left = n.borrow().left.clone();
            //         n.borrow_mut().left = self.node_delete(left, delete_value);
            //         n
            //     }
            //     else if delete_value > node_value { // look right
            //         let right = n.borrow().right.clone();
            //         n.borrow_mut().right = self.node_delete(right, delete_value);
            //         n
            //     }
            //     else { // found the node which should be deleted
            //         let left = n.borrow().left.clone();
            //         let right = n.borrow().right.clone();
            //         let ret = match (left.clone(), right.clone()) {
            //             (None, Some(r)) => r,  // The left subtree of the node to be deleted is empty, r is new root
            //             (Some(l), None) => l, // The right subtree of the node to be deleted is empty, l is new root
            //
            //             // 我想通过左子树为空/右子树为空/左右都不为空，这三种情况进行分类，但是缺少左右子树都为空
            //             // 如果把(None, Some(r)) => r，改成(None, _) => right.unwrap()???，或者改成(_, Some(r)) => r???
            //             (None, None) => None.unwrap(),
            //
            //             //The left and right subtrees of the node to be deleted(node n) are not empty.
            //             // Find the smallest node A that is larger than the node n.
            //             (Some(left), Some(right)) => {
            //                 let min_value = right.borrow().min(); // Find the value of node A which is the minimum value of the right subtree
            //                 n.borrow_mut().value = min_value; // Change the value of node n to the value of node A.
            //                 let right = n.borrow().right.clone().take();
            //                 n.borrow_mut().right = self.node_delete(right, min_value); // Delete the node A in the right subtree.
            //                 n // return new root
            //             },
            //         };
            //         ret
            //     }
            // },
            None => node, // 遍历到叶子节点，但是还是没有找到，所以应该返回null，还是说因为叶子节点就是null，所以返回node就可以？？？
            Some(mut n) => {
                let node_value = n.borrow().value;
                if delete_value < node_value  { // look left
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = self.node_delete(left, delete_value);
                    Some(n) // 返回option
                }
                else if delete_value > node_value { // look right
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = self.node_delete(right, delete_value);
                    Some(n) // 返回option
                }
                else { // found the node which should be deleted
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();
                    let ret = match (left.clone(), right.clone()) {
                        (None, Some(r)) => Some(r),  // The left subtree of the node to be deleted is empty, r is new root
                        (Some(l), None) => Some(l), // The right subtree of the node to be deleted is empty, l is new root

                        // 我想通过左子树为空/右子树为空/左右都不为空，这三种情况进行分类，但是缺少左右子树都为空
                        // 如果把(None, Some(r)) => r，改成(None, _) => right.unwrap()???，或者改成(_, Some(r)) => r???
                        (None, None) => None,

                        //The left and right subtrees of the node to be deleted(node n) are not empty.
                        // Find the smallest node A that is larger than the node n.
                        (Some(left), Some(right)) => {
                            let min_value = right.borrow().min(); // Find the value of node A which is the minimum value of the right subtree
                            n.borrow_mut().value = min_value; // Change the value of node n to the value of node A.
                            let right = n.borrow().right.clone().take();
                            n.borrow_mut().right = self.node_delete(right, min_value); // Delete the node A in the right subtree.
                            Some(n) // return new root
                        },
                    };
                    ret // 返回option
                }
            },
        };

        // update and maintain
        match ret_node {
            None => ret_node,
            Some(n) => {
                // update height
                n.borrow_mut().height = self.get_left_height(&n) // 借用了发生移动的
                    .max(self.get_right_height(&n)) + 1; // 把option类型的ret_node都改成了n

                // update balance factor
                let mut balance_factor = self.get_balance_factor(&n);
                // if balance_factor.abs() > 1.0 {
                //     println!("unbalanced: {}", balance_factor);
                // }

                // maintain
                // case LL: right rotate
                if balance_factor > 1.0 && self.get_balance_factor(&n.borrow().left.clone().unwrap()) >= 0.0 {
                    return Some(self.right_rotate(n))
                }

                // case RR: left rotate
                if balance_factor < -1.0 && self.get_balance_factor(&n.borrow().right.clone().unwrap()) <= 0.0 {
                    return Some(self.left_rotate(n))
                }

                // case LR: left rotate + right rotate
                if balance_factor > 1.0 && self.get_balance_factor(&n.borrow().left.clone().unwrap()) < 0.0 {
                    // ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
                    // return Some(self.right_rotate(ret_node))

                    let left = n.borrow().left.clone().take().unwrap();
                    n.borrow_mut().left = Some(self.left_rotate(left));
                    return Some(self.right_rotate(n))
                }

                // case RL: right rotate + left rotate
                if balance_factor < -1.0 && self.get_balance_factor(&n.borrow().right.clone().unwrap()) > 0.0 {
                    // ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
                    // return Some(self.left_rotate(ret_node))

                    let right = n.borrow().right.clone().take().unwrap();
                    n.borrow_mut().right = Some(self.right_rotate(right));
                    return Some(self.left_rotate(n))
                }
                Some(n)
            }
        }
    }

    /// Return the minimum value of the AVL tree
    ///
    /// # Example
    ///
    /// ```
    /// use trees::bstree::BinarySearchTree;
    /// use trees::base::QueryableTree;
    ///
    /// let mut tree = BinarySearchTree::new();
    /// println!("{:?}", tree.min());  // None
    /// tree.insert(1);
    /// tree.insert(0);
    /// tree.insert(5);
    /// tree.insert(3);
    /// tree.insert(2);
    /// println!("{:?}", tree.min());  // Some(0)
    /// ```
    pub fn min(&self) -> Option<u32> {
        match self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().min()),
        }
    }

    // Helper functions
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
        let mut T3 = x.borrow().right.clone().take();

        // right rotate
        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().left = T3; // 借用了发生移动的y

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
        // let mut T2 = x.borrow().left.clone().unwrap(); // 在这里会Panic，因为在21345情况下，4的左子树T2是none，这就和类型不对应了
        let mut T2 = x.borrow().left.clone().take(); // 这样T2是option类型就可以处理none的情况

        // left rotate
        x.borrow_mut().left = Some(y.clone());
        //y.borrow_mut().right = Some(T2); // 借用了发生移动的y
        y.borrow_mut().right = T2;

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y)
            .max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x)
            .max(self.get_right_height(&x)) + 1;

        return x
    }
}

fn main() {
    let mut avl_tree = AVLTree::new();

    // // case LL: right rotate
    // avl_tree.insert(1);
    // avl_tree.insert(2);
    // avl_tree.insert(3);
    //
    // // case RR: left rotate
    // avl_tree.insert(3);
    // avl_tree.insert(2);
    // avl_tree.insert(1);
    //
    // case LR: left rotate + right rotate
    // avl_tree.insert(3);
    // avl_tree.insert(1);
    // avl_tree.insert(2);
    //
    // case RL: right rotate + left rotate
    // avl_tree.insert(1);
    // avl_tree.insert(3);
    // avl_tree.insert(2);
    //
    // avl_tree.delete(1); // 树里剩2和3，叶子节点数应为3，得到2错误，高度3正确
    // avl_tree.delete(2); // 树里只剩3了，叶子数2，高度2正确
    // avl_tree.delete(3); //正确

    avl_tree.insert(1);
    avl_tree.insert(2);
    avl_tree.insert(3);
    avl_tree.insert(4);
    avl_tree.insert(5);

    avl_tree.delete(1);//正确
    avl_tree.delete(2); //正确
    avl_tree.delete(3); //树里剩4和5，叶子节点数应为3，得到2错误，高度3正确
    avl_tree.delete(4);//正确
    avl_tree.delete(5); // 0 0 true 正确

    avl_tree.in_order_traversal();
    println!("Count leaves: {:?}", avl_tree.count_leaves());
    println!("Height: {:?}", avl_tree.height());
    println!("Is empty: {:?}", avl_tree.is_tree_empty());
}

// The functions that can be tested are insert() case LL, case RR, case LR, case RL, delete()
// in_order_traversal(), height() and .is_tree_empty().
// When there are only two nodes in the tree, the number of leaf nodes should be 3 and count_leaves() returns 2.
// In other cases count_leaves() is correct.
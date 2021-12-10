use std::cell::RefCell;
use std::cmp::{max, Ordering};
use std::fmt;
use std::io;
use std::rc::Rc;

pub use crate::commonTrait::{CommonTreeNodeTrait, CommonTreeTrait};

type AVLTreeNode<T> = Rc<RefCell<TreeNode<T>>>;
type OptionAVLTreeNode<T> = Option<AVLTreeNode<T>>;

#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode<T: Ord + Copy + fmt::Debug> {
    pub value: T,
    left: OptionAVLTreeNode<T>,
    right: OptionAVLTreeNode<T>,
    height: usize,
}

// extend from common tree trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeTrait<T, TreeNode<T>> for AVLTree<T> {
    fn get_root(&self) -> OptionAVLTreeNode<T> {
        return self.root.clone();
    }
}

// extend from common tree node trait
impl<T: Ord + Copy + fmt::Debug> CommonTreeNodeTrait<T> for TreeNode<T> {
    fn get_left(&self) -> OptionAVLTreeNode<T> {
        return self.left.clone();
    }

    fn get_right(&self) -> OptionAVLTreeNode<T> {
        return self.right.clone();
    }

    fn get_value(&self) -> T {
        return self.value;
    }
}

impl<T: Ord + Copy + fmt::Debug> TreeNode<T> {
    // Create a new node of type OptionAVLTreeNode
    fn new(value: T) -> OptionAVLTreeNode<T> {
        Some(Rc::new(RefCell::new(Self {
            value,
            left: None,
            right: None,
            height: 1, // default height of a new node is 1，which is a leave
        })))
    }

    fn get_data(&self) -> T {
        return self.value;
    }

    // fn min(&self) -> T {
    //     self.get_left()
    //         .map_or(self.get_data(), |x| x.borrow_mut().min())
    // }

    // // Node有height属性，可以简化？
    // fn get_height(node: AVLTreeNode<T>) -> u32 {
    //     let left = node.borrow().left.clone();
    //     let right = node.borrow().right.clone();
    //     let left_height = left.map(|l| Self::get_height(l.clone())).unwrap_or(1);
    //     let right_height = right.map(|r| Self::get_height(r.clone())).unwrap_or(1);
    //     return max(left_height, right_height) + 1;
    // }

    // fn in_order_traversal(node: AVLTreeNode<T>) {
    //     let left = node.borrow().left.clone();
    //     if left.is_some() {
    //         Self::in_order_traversal(left.unwrap());
    //     }
    //     print!("{:?} ", node.borrow().value);
    //     let right = node.borrow().right.clone();
    //     if right.is_some() {
    //         Self::in_order_traversal(right.unwrap());
    //     }
    // }

    // fn pre_order_traversal(node: AVLTreeNode<T>) {
    //     print!("{:?}", node.borrow().value);
    //     let left = node.borrow().left.clone();
    //     if left.is_some() {
    //         Self::pre_order_traversal(left.unwrap());
    //     }
    //     let right = node.borrow().right.clone();
    //     if right.is_some() {
    //         Self::pre_order_traversal(right.unwrap());
    //     }
    // }
}

pub struct AVLTree<T: Ord + Copy + fmt::Debug> {
    root: OptionAVLTreeNode<T>,
}

impl<T: Ord + Copy + fmt::Debug> AVLTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    // // count the leaves (None nodes)
    // pub fn count_leaves(&self) -> u32 {
    //     match self.root.clone() {
    //         None => 0,
    //         Some(node) => TreeNode::count_leaves(node),
    //     }
    // }

    // // from root to leaves
    // pub fn height(&self) -> u32 {
    //     match self.root.clone() {
    //         None => 0,
    //         Some(node) => TreeNode::get_height(node),
    //     }
    // }

    pub fn preorder_traverse(&self, node: AVLTreeNode<T>, container: &mut Vec<T>) {
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

    pub fn in_order_traverse(&self, node: AVLTreeNode<T>, container: &mut Vec<T>) {
        let left = node.borrow().left.clone();
        if left.is_some() {
            self.in_order_traverse(left.unwrap(), container);
        }
        container.push(node.borrow().value);
        let right = node.borrow().right.clone();
        if right.is_some() {
            self.in_order_traverse(right.unwrap(), container);
        }
    }

    // pub fn pre_order_traversal(&self) {
    //     print!("Preorder traversal: ");
    //     match self.root.clone() {
    //         None => print!("the tree does not have node"),
    //         Some(root) => TreeNode::pre_order_traversal(root),
    //     }
    //     println!();
    // }

    // // inorder traverse
    // pub fn in_order_traversal(&self) {
    //     print!("Inorder traversal: ");
    //     match self.root.clone() {
    //         None => print!("the tree does not have node"),
    //         Some(root) => TreeNode::in_order_traversal(root),
    //     }
    //     println!()
    // }

    // judge if the tree is empty
    pub fn is_tree_empty(&self) -> bool {
        self.root.clone().map(|_| false).unwrap_or(true)
    }

    pub fn insert(&mut self, insert_value: T) {
        let root = self.root.take();
        // TreeNode is type OptionAVLTreeNode, so the code is simplified.
        match root {
            None => self.root = TreeNode::new(insert_value),
            Some(n) => self.root = self.node_insert(Some(n), insert_value),
        }
    }

    fn node_insert(&mut self, node: OptionAVLTreeNode<T>, insert_value: T) -> OptionAVLTreeNode<T> {
        let ret_node = match node {
            Some(mut n) => {
                let node_value = n.borrow().value;
                if insert_value < node_value {
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
        ret_node.borrow_mut().height = self
            .get_left_height(&ret_node)
            .max(self.get_right_height(&ret_node))
            + 1;

        // update balance factor
        let mut balance_factor = self.get_balance_factor(&ret_node);
        // if balance_factor.abs() > 1.0 {
        //     println!("unbalanced: {}", balance_factor);
        // }

        //println!("insert node: {}", ret_node.borrow().value.clone());

        // maintain
        // case LL: right rotate
        if balance_factor > 1.0
            && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) >= 0.0
        {
            return Some(self.right_rotate(ret_node));
        }

        // case RR: left rotate
        if balance_factor < -1.0
            && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) <= 0.0
        {
            return Some(self.left_rotate(ret_node));
        }

        // case LR: left rotate + right rotate
        if balance_factor > 1.0
            && self.get_balance_factor(&ret_node.borrow().left.clone().unwrap()) < 0.0
        {
            // ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
            // return Some(self.right_rotate(ret_node))

            let left = ret_node.borrow().left.clone().take().unwrap();
            ret_node.borrow_mut().left = Some(self.left_rotate(left));
            return Some(self.right_rotate(ret_node));
        }

        // case RL: right rotate + left rotate
        if balance_factor < -1.0
            && self.get_balance_factor(&ret_node.borrow().right.clone().unwrap()) > 0.0
        {
            // ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
            // return Some(self.left_rotate(ret_node))

            let right = ret_node.borrow().right.clone().take().unwrap();
            ret_node.borrow_mut().right = Some(self.right_rotate(right));
            return Some(self.left_rotate(ret_node));
        }
        Some(ret_node)
    }

    pub fn delete(&mut self, delete_value: T) {
        let root = self.root.take();
        match root {
            None => return, // 这棵树是空的，没办法delete，所以什么都没发生??? 还是应该返回null？？？？？？？
            Some(n) => self.root = self.node_delete(Some(n), delete_value),
        }
    }

    // delete node, return new root
    fn node_delete(&mut self, node: OptionAVLTreeNode<T>, delete_value: T) -> OptionAVLTreeNode<T> {
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
                if delete_value < node_value {
                    // look left
                    let left = n.borrow().left.clone();
                    n.borrow_mut().left = self.node_delete(left, delete_value);
                    Some(n) // 返回option
                } else if delete_value > node_value {
                    // look right
                    let right = n.borrow().right.clone();
                    n.borrow_mut().right = self.node_delete(right, delete_value);
                    Some(n) // 返回option
                } else {
                    // found the node which should be deleted
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();
                    let ret = match (left.clone(), right.clone()) {
                        (None, Some(r)) => Some(r), // The left subtree of the node to be deleted is empty, r is new root
                        (Some(l), None) => Some(l), // The right subtree of the node to be deleted is empty, l is new root

                        // 我想通过左子树为空/右子树为空/左右都不为空，这三种情况进行分类，但是缺少左右子树都为空
                        // 如果把(None, Some(r)) => r，改成(None, _) => right.unwrap()???，或者改成(_, Some(r)) => r???
                        (None, None) => None,

                        //The left and right subtrees of the node to be deleted(node n) are not empty.
                        // Find the smallest node A that is larger than the node n.
                        (Some(left), Some(right)) => {
                            let min_value = right.borrow().get_min_value_in_children(); // Find the value of node A which is the minimum value of the right subtree
                            n.borrow_mut().value = min_value; // Change the value of node n to the value of node A.
                            let right = n.borrow().right.clone().take();
                            n.borrow_mut().right = self.node_delete(right, min_value); // Delete the node A in the right subtree.
                            Some(n) // return new root
                        }
                    };
                    ret // 返回option
                }
            }
        };

        // update and maintain
        match ret_node {
            None => ret_node,
            Some(n) => {
                // update height
                n.borrow_mut().height = self
                    .get_left_height(&n) // 借用了发生移动的
                    .max(self.get_right_height(&n))
                    + 1; // 把option类型的ret_node都改成了n

                // update balance factor
                let mut balance_factor = self.get_balance_factor(&n);
                // if balance_factor.abs() > 1.0 {
                //     println!("unbalanced: {}", balance_factor);
                // }

                // maintain
                // case LL: right rotate
                if balance_factor > 1.0
                    && self.get_balance_factor(&n.borrow().left.clone().unwrap()) >= 0.0
                {
                    return Some(self.right_rotate(n));
                }

                // case RR: left rotate
                if balance_factor < -1.0
                    && self.get_balance_factor(&n.borrow().right.clone().unwrap()) <= 0.0
                {
                    return Some(self.left_rotate(n));
                }

                // case LR: left rotate + right rotate
                if balance_factor > 1.0
                    && self.get_balance_factor(&n.borrow().left.clone().unwrap()) < 0.0
                {
                    // ret_node.borrow_mut().left = Some(self.left_rotate(ret_node.borrow_mut().left.clone().unwrap())); // 发生移动
                    // return Some(self.right_rotate(ret_node))

                    let left = n.borrow().left.clone().take().unwrap();
                    n.borrow_mut().left = Some(self.left_rotate(left));
                    return Some(self.right_rotate(n));
                }

                // case RL: right rotate + left rotate
                if balance_factor < -1.0
                    && self.get_balance_factor(&n.borrow().right.clone().unwrap()) > 0.0
                {
                    // ret_node.borrow_mut().right = Some(self.right_rotate(ret_node.borrow_mut().right.clone().unwrap())); // 发生移动
                    // return Some(self.left_rotate(ret_node))

                    let right = n.borrow().right.clone().take().unwrap();
                    n.borrow_mut().right = Some(self.right_rotate(right));
                    return Some(self.left_rotate(n));
                }
                Some(n)
            }
        }
    }

    // fn get_root(&self) -> &OptionAVLTreeNode<T> {
    //     &self.root
    // }

    fn get_height(&self, node: OptionAVLTreeNode<T>) -> usize {
        // default height of an empty tree is 0
        node.map_or(0, |n| n.borrow().height)
    }
    fn get_left_height(&self, n: &AVLTreeNode<T>) -> usize {
        self.get_height(n.borrow().left.clone())
    }

    fn get_right_height(&self, n: &AVLTreeNode<T>) -> usize {
        self.get_height(n.borrow().right.clone())
    }

    fn get_balance_factor(&self, n: &AVLTreeNode<T>) -> f64 {
        self.get_left_height(n) as f64 - self.get_right_height(n) as f64
    }

    //Determine whether the tree is balanced
    fn is_balanced(&self, node: OptionAVLTreeNode<T>) -> bool {
        match node {
            Some(node) => {
                if self.get_balance_factor(&node) <= 1.0 {
                    self.is_balanced(node.borrow().left.clone())
                        && self.is_balanced(node.borrow().right.clone())
                } else {
                    false
                }
            }
            None => true,
        }
    }

    //                 y                                     x
    //               /    \                                 /   \
    //              x     T4      right rotate (y)         z     y
    //           /   \            ---------------->       / \   / \
    //          z     T3             return x            T1 T2 T3 T4
    //         /   \
    //        T1   T2
    fn right_rotate(&self, y: AVLTreeNode<T>) -> AVLTreeNode<T> {
        let mut x = y.borrow().left.clone().unwrap();
        let mut T3 = x.borrow().right.clone().take();

        // right rotate
        x.borrow_mut().right = Some(y.clone());
        y.borrow_mut().left = T3; // 借用了发生移动的y

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y).max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x).max(self.get_right_height(&x)) + 1;

        return x;
    }

    //                 y                                        x
    //               /    \                                   /   \
    //              T1     x        left rotate (y)          y     z
    //                   /   \      ---------------->       / \   / \
    //                 T2     z        return x            T1 T2 T3 T4
    //                       /  \
    //                      T3   T4
    fn left_rotate(&self, y: AVLTreeNode<T>) -> AVLTreeNode<T> {
        let mut x = y.borrow().right.clone().unwrap();
        // let mut T2 = x.borrow().left.clone().unwrap(); // 在这里会Panic，因为在21345情况下，4的左子树T2是none，这就和类型不对应了
        let mut T2 = x.borrow().left.clone().take(); // 这样T2是option类型就可以处理none的情况

        // left rotate
        x.borrow_mut().left = Some(y.clone());
        //y.borrow_mut().right = Some(T2); // 借用了发生移动的y
        y.borrow_mut().right = T2;

        // update height of x and y
        y.borrow_mut().height = self.get_left_height(&y).max(self.get_right_height(&y)) + 1;
        x.borrow_mut().height = self.get_left_height(&x).max(self.get_right_height(&x)) + 1;

        return x;
    }
}

// The functions that can be tested are insert() case LL, case RR, case LR, case RL, delete()
// in_order_traversal(), height() and .is_tree_empty().
// When there are only two nodes in the tree, the number of leaf nodes should be 3 and count_leaves() returns 2.
// In other cases count_leaves() is correct.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_traversal() {
        // Test the three different tree traversal functions.
        let mut tree = AVLTree::new();
        tree.insert(0);
        vec![16, 16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });
        let root = tree.root.clone().unwrap();
        let mut pre_container = vec![];
        let mut in_container = vec![];
        tree.preorder_traverse(root.clone(), &mut pre_container);
        tree.in_order_traverse(root.clone(), &mut in_container);
        let is_balanced = tree.is_balanced(tree.root.clone());
        // println!("check {:#?}", in_container);
        assert_eq!(pre_container, vec![20, 8, 0, 16, 24, 22]);
        assert_eq!(in_container, vec![0, 8, 16, 20, 22, 24]);
        assert_eq!(is_balanced, true);
    }

    #[test]
    fn test_insert() {
        let mut avl_tree = AVLTree::new();
        avl_tree.insert(1);
        avl_tree.insert(2);
        avl_tree.insert(3);
        avl_tree.insert(4);
        avl_tree.insert(5);

        let result = avl_tree.is_balanced(avl_tree.root.clone());
        assert_eq!(result, true);
    }

    #[test]
    fn test_delete() {
        // Test the three different tree traversal functions.
        let mut tree = AVLTree::new();
        tree.insert(0);
        vec![16, 8, 24, 20, 22].iter().for_each(|v| {
            tree.insert(*v);
        });

        let root = tree.root.clone().unwrap();
        tree.delete(16);
        let mut container = vec![];
        tree.preorder_traverse(root.clone(), &mut container);
        let result = tree.is_balanced(tree.root.clone());
        assert_eq!(result, true);

        assert_eq!(container, vec![20, 8, 0, 24, 22]);
    }
}

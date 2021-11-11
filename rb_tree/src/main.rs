use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

type RBTreeNode = Option<Rc<RefCell<TreeNode<u32>>>>;

#[derive(Clone, Debug, PartialEq)]
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RBTreeNode,
    left: RBTreeNode,
    right: RBTreeNode,
}

#[derive(Clone, Debug, PartialEq)]
struct RBTree {
    root: RBTreeNode,
    length: u32,
}

impl RBTree {
    fn new() -> Self {
        RBTree {
            root: None,
            length: 0,
        }
    }

    fn insert(&mut self, insert_node: RBTreeNode) {
        
    }

    // TODO: panic user if node is not root
    fn node_insert(&mut self, node: RBTreeNode, insert_node: TreeNode<u32>) -> RBTree {
        
    }

    // find the min node
    fn find_min(&self, node: RBTreeNode) -> Option<RBTreeNode> {
        // match node {
        //     None => None,
        //     Some(inner) => {
        //         if inner.borrow().left.is_some() {
        //             self.find_min(inner.borrow().left)
        //         } else {
        //             Some(inner.borrow().clone())
        //         }
        //     }
        // }
    }

    fn delete(&mut self, key: u32) {
        
    }

    fn node_delete(&mut self, node: RBTreeNode, key: u32) -> RBTreeNode {

    }

    fn count_leaves(&self) -> u32 {
        
    }

    fn height(&self) -> u32 {

    }

    fn in_order_traversal(&self) {

    }

    fn is_tree_empty(&self) -> bool {

    }
}

fn main() {

}



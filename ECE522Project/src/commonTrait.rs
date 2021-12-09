use std::cell::RefCell;
use std::cmp::max;
use std::fmt;
use std::rc::Rc;

// Common trait for Tree
pub trait CommonTreeTrait<T: Ord + Copy + fmt::Debug, TreeNode: CommonTreeNodeTrait<T>> {
    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>>;

    fn count_leaves(&self) -> u32 {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    fn height(&self) -> u32 {
        match self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }

    fn in_order_traversal(&self) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().in_order_traversal();
                println!();
            }
        }
    }

    fn pre_order_traversal(&self) {
        match self.get_root() {
            None => println!("There is no node in the tree!"),
            Some(node) => {
                node.borrow().pre_order_traversal();
                println!();
            }
        }
    }

    fn contains(&self, value: T) -> bool {
        match self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }
}

// Common trait for TreeNode
pub trait CommonTreeNodeTrait<T: Ord + Copy + fmt::Debug> {
    fn get_left(&self) -> Option<Rc<RefCell<Self>>>;

    fn get_right(&self) -> Option<Rc<RefCell<Self>>>;

    fn get_value(&self) -> T;

    fn count_leaves(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        if left.is_none() && right.is_none() {
            return 1;
        } else if left.is_none() && right.is_some() {
            return right.unwrap().borrow().count_leaves();
        } else if left.is_some() && right.is_none() {
            return left.unwrap().borrow().count_leaves();
        } else {
            return right.unwrap().borrow().count_leaves() + left.unwrap().borrow().count_leaves();
        }
    }

    fn get_height(&self) -> u32 {
        let left = self.get_left();
        let right = self.get_right();
        let left_height = match left {
            None => 0,
            Some(l) => l.borrow().get_height(),
        };
        let right_height = match right {
            None => 0,
            Some(r) => r.borrow().get_height(),
        };
        return max(left_height, right_height) + 1;
    }

    fn in_order_traversal(&self) {
        let left = self.get_left();
        if left.is_some() {
            left.unwrap().borrow().in_order_traversal();
        }
        print!("{:?} ", self.get_value());
        let right = self.get_right();
        if right.is_some() {
            right.unwrap().borrow().in_order_traversal();
        }
    }

    fn pre_order_traversal(&self) {
        print!("{:?} ", self.get_value());
        let left = self.get_left();
        if left.is_some() {
            left.unwrap().borrow().in_order_traversal();
        }
        let right = self.get_right();
        if right.is_some() {
            right.unwrap().borrow().in_order_traversal();
        }
    }

    fn contains(&self, value: T) -> bool {
        let current_value = self.get_value();
        return if current_value == value {
            true
        } else if current_value > value {
            match self.get_left() {
                None => false,
                Some(node) => node.borrow().contains(value),
            }
        } else {
            match self.get_right() {
                None => false,
                Some(node) => node.borrow().contains(value),
            }
        };
    }
}

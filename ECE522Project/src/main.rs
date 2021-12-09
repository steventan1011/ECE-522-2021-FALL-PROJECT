pub mod avlTree;
pub mod bsTree;
pub mod commonTrait;
pub mod rbTree;

pub use crate::avlTree::AVLTree;
// pub use crate::bsTree::BSTree;
pub use crate::commonTrait::{CommonTreeNodeTrait, CommonTreeTrait};
pub use crate::rbTree::RBTree;

use std::io::{stdin, stdout, Write};

pub fn get_user_input() -> String {
    let mut line = String::new();
    stdout().flush().expect("failed to flush");
    stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");
    line.to_string()
}
pub fn get_val(op: &str) -> u32 {
    loop {
        print!("{} value$ ", op);
        let value = get_user_input();
        let trimmed_val = value.trim();
        match trimmed_val.parse::<u32>() {
            Ok(val) => {
                println!("{} value '{}' in tree ... done!", op, val);
                return val;
            }
            Err(..) => {
                println!("this was not an integer number");
            }
        };
    }
}

pub fn avl_promote() {
    println!("\n::--------------AVL Tree--------------::\n");
    let mut tree = AVLTree::new();

    loop {
        print!("operation$ ");
        let operation = get_user_input();

        match operation.to_lowercase().trim() {
            "insert" => {
                let val = get_val("insert");
                tree.insert(val);
            }
            "delete" => {
                let val = get_val("delete");
                tree.delete(val);
            }

            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),

            "height" => println!("Height of tree: {:?}", tree.height()),

            "inorder print" => {
                print!("Your tree: ");
                tree.in_order_traversal();
            },

            "empty" => println!("Is the tree empty?: {:?}", tree.is_tree_empty()),

            "search" => {
                let val = get_val("search");
                // println!("values found? {:?}", tree.contains(val));
            },

            "print tree" => println!("Your tree: "),
            
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}


fn rbt_promote() {
    println!("\n::---------------Red-Black Tree--------------::\n");
    let mut tree = RBTree::new();

    loop {
        print!("operation$ ");
        let operation = get_user_input();

        match operation.to_lowercase().trim() {
            "insert" => {
                let val = get_val("insert");
                tree.insert(val);
            }
            "delete" => {
                let val = get_val("delete");
                tree.delete(val);
            },
            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),

            "height" => println!("Height of tree: {:?}", tree.height()),

            "inorder print" => {
                print!("Your tree: ");
                tree.in_order_traversal();
            },

            "empty" => println!("Is the tree empty?: {:?}", tree.is_tree_empty()),

            "search" => {
                let val = get_val("search");
                // println!("values found? {:?}", tree.contains(val));
            },

            "print tree" => println!("Your tree: "),
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}

fn main() {
    println!("Hello!");
    println!("you can select a tree to start or type in 'exit' to leave");
    println!("Select a tree!\n-AVLTree \n-RBTree or type 'help' to learn about the commands");
    print!("input$ ");
    let selected_tree = get_user_input();

    match selected_tree.to_lowercase().trim() {
        "avltree" => {
            avl_promote();
        },
        "rbtree" => {
            rbt_promote();
        },
        _ => {
            eprint!("Command not recognized. \n");
        }
    }
}

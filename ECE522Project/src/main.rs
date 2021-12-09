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
        print!("{} value > ", op);
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

pub fn avl_cli() {
    println!("\n::...AVL Tree branch...::\n");
    let mut tree = AVLTree::new();
    // list_of_operations();

    loop {
        print!("operation > ");
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

            // "contain" | "search" => {
            //     let val = get_val("search");
            //     println!("values found? {:?}", tree.contains(val));
            // },
            "height" => println!("Height of tree: {:?}", tree.height()),
            "count" => println!("Number of leaves: {:?}", tree.count_leaves()),
            // "length" => println!("Length: {:?}", tree.len()),
            "min" => {
                let min_val = tree.min();
                match min_val {
                    None => println!("It is an empty tree!"),
                    Some(v) => println!("Minimum Value: {:?}", v),
                }
            }
            // "max" => {
            //     let max_val = tree.max();
            //     match max_val {
            //         None => println!("It is an empty tree!"),
            //         Some(v) => println!("Maximum Value: {:?}", v),
            //     }
            // },
            // "empty" => println!("Is the tree empty?: {:?}", tree.is_empty()),
            "print" => {
                print!("Your tree: ");
                tree.in_order_traversal();
            }
            // "help" => list_of_operations(),
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}

fn rbt_cli() {
    println!("\n::...Red-Black Tree branch...::\n");
    let mut tree = RBTree::new();
    // list_of_operations();

    loop {
        print!("operation > ");
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
            "print" => {
                print!("Your tree: ");
                tree.pre_order_traversal();
            }
            "exit" => return,
            _ => println!("Command not recognized. Try 'help' for valid operations"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("you can select a tree to start or print 'exit' to leave");
    println!("Select a tree!\n-AVL \n-RBT or type 'help' to learn about the commands");
    print!("input > ");
    let selected_tree = get_user_input();

    match selected_tree.to_lowercase().trim() {
        "avl" => {
            avl_cli();
        }
        "rbt" => {
            rbt_cli();
        }
        _ => {
            eprint!("Command not recognized. \n");
        }
    }
}

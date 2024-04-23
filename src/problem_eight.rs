use std::io;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn build_tree() -> Option<Box<TreeNode>> {
    println!("Enter the value of the root node:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let root_val: i32 = input.trim().parse().expect("Invalid input for root node");

    if root_val == -1 {
        return None;
    }

    let mut root = TreeNode::new(root_val);
    println!("Enter the left subtree of {} (enter -1 for no node):", root_val);
    root.left = build_tree();
    println!("Enter the right subtree of {} (enter -1 for no node):", root_val);
    root.right = build_tree();

    Some(Box::new(root))
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

pub fn main() {
    println!("8. Given a binary tree, implement a function that returns the maximum depth of the tree.");
    println!("Construct the binary tree:");
    let root = build_tree();

    println!("Maximum depth of the tree: {}", max_depth(root));
}

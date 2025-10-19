// exercises/trees/tree_node_definition.rs
// Make me pass the test!
// Define the basic TreeNode structure and implement its `new` function.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // Implement the 'new' function to create a new TreeNode.
    // It should take an integer value and return a new TreeNode with
    // its `val` set to the given value and `left` and `right` set to `None`.
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = TreeNode::new(5);
        assert_eq!(node.val, 5);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_new_node_zero() {
        let node = TreeNode::new(0);
        assert_eq!(node.val, 0);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }

    #[test]
    fn test_new_node_negative() {
        let node = TreeNode::new(-10);
        assert_eq!(node.val, -10);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
    }
}

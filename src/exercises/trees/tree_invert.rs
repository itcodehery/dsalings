// exercises/trees/tree_invert.rs
// Make me pass the test!
//
// Exercise: Invert Binary Tree
// Given the `root` of a binary tree, invert the tree, and return its root.
// Inverting a binary tree means that for every node, its left and right children are swapped.
//
// For example:
// Input: root = [4,2,7,1,3,6,9]
// Output: [4,7,2,9,6,3,1]
//
// (Visualization: A tree where left and right subtrees are mirrored at every node.)

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // TODO: Implement the function to invert the binary tree.
    // Hints:
    // 1. This problem can be solved recursively or iteratively.
    // 2. For a recursive approach:
    //    a. The base case is when the node is None.
    //    b. For a non-None node, swap its left and right children.
    //    c. Recursively call `invert_tree` on the new left child and the new right child.
    // 3. For an iterative approach, you might use a queue (BFS) to process nodes level by level.

    root // Placeholder, make this return the inverted tree root
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a tree from a vector of optional values (BFS order)
    // None represents a null node.
    fn create_tree_from_vec(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root_val = values[0].unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;

        while let Some(node_rc) = queue.pop_front() {
            let mut node = node_rc.borrow_mut();

            if i < values.len() {
                if let Some(val) = values[i] {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
                i += 1;
            }

            if i < values.len() {
                if let Some(val) = values[i] {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
                i += 1;
            }
        }
        Some(root)
    }

    #[test]
    fn test_invert_empty_tree() {
        let root = None;
        let inverted = invert_tree(root);
        assert_eq!(inverted, None);
    }

    #[test]
    fn test_invert_single_node_tree() {
        let root = create_tree_from_vec(&[Some(1)]);
        let expected = create_tree_from_vec(&[Some(1)]);
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_example_tree_1() {
        // Original: [4,2,7,1,3,6,9]
        let root = create_tree_from_vec(&[
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        // Expected: [4,7,2,9,6,3,1]
        let expected = create_tree_from_vec(&[
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_example_tree_2() {
        // Original: [2,1,3]
        let root = create_tree_from_vec(&[Some(2), Some(1), Some(3)]);
        // Expected: [2,3,1]
        let expected = create_tree_from_vec(&[Some(2), Some(3), Some(1)]);
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_tree_with_null_children() {
        // Original: [1,2,null,3]
        let root = create_tree_from_vec(&[Some(1), Some(2), None, Some(3)]);
        // Expected: [1,null,2,null,null,null,3]
        // This effectively means 1 -> right child 2, 2 -> right child 3
        let expected = create_tree_from_vec(&[Some(1), None, Some(2), None, None, Some(3), None]);
        assert_eq!(invert_tree(root), expected);
    }

    #[test]
    fn test_invert_complex_tree() {
        // Original: [1,2,3,4,null,null,5,6,null,null,7]
        let root = create_tree_from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            Some(6),
            None,
            None,
            Some(7),
        ]);
        // Expected: [1,3,2,5,null,null,4,null,7,null,6]
        let expected = create_tree_from_vec(&[
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            None,
            None,
            Some(4),
            None,
            Some(7),
            None,
            Some(6),
        ]);
        assert_eq!(invert_tree(root), expected);
    }
}

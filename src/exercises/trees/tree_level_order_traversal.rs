// exercises/trees/tree_level_order_traversal.rs
// Make me pass the test!
//
// Exercise: Binary Tree Level Order Traversal
// Given the `root` of a binary tree, return the level order traversal of its nodes' values.
// (i.e., from root to left, then right, level by level).
//
// For example:
// Input: root = [3,9,20,null,null,15,7]
// Output: [[3],[9,20],[15,7]]
//
// (Visualization: A breadth-first search (BFS) approach is typically used for this.)

use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    // TODO: Implement the function to perform a level-order traversal of the binary tree.
    // Hints:
    // 1. Use a queue (e.g., `std::collections::VecDeque`) for a breadth-first search (BFS).
    // 2. Start by pushing the root node (if it exists) into the queue.
    // 3. In each iteration, process all nodes at the current level.
    //    a. Determine the number of nodes at the current level (the current queue size).
    //    b. Create a vector to store the values of nodes at the current level.
    //    c. Dequeue each node, add its value to the current level's vector.
    //    d. Enqueue its left and right children (if they exist).
    // 4. Add the current level's vector to the result.
    // 5. Continue until the queue is empty.

    vec![] // Placeholder, make this return the correct level order traversal
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a tree from a vector of optional values (BFS order)
    fn create_tree_from_vec(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root_val = values[0].unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = VecDeque::new();
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
    fn test_level_order_empty_tree() {
        let root = None;
        assert_eq!(level_order(root), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_level_order_single_node_tree() {
        let root = create_tree_from_vec(&[Some(1)]);
        assert_eq!(level_order(root), vec![vec![1]]);
    }

    #[test]
    fn test_level_order_example_1() {
        // Input: root = [3,9,20,null,null,15,7]
        let root =
            create_tree_from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(level_order(root), expected);
    }

    #[test]
    fn test_level_order_example_2() {
        // Input: root = [1]
        let root = create_tree_from_vec(&[Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(level_order(root), expected);
    }

    #[test]
    fn test_level_order_example_3() {
        // Input: root = []
        let root = create_tree_from_vec(&[]);
        assert_eq!(level_order(root), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_level_order_complete_tree() {
        let root = create_tree_from_vec(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);
        let expected = vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]];
        assert_eq!(level_order(root), expected);
    }

    #[test]
    fn test_level_order_incomplete_tree() {
        let root = create_tree_from_vec(&[Some(1), Some(2), None, Some(3), None, None, Some(4)]);
        let expected = vec![vec![1], vec![2], vec![3], vec![4]];
        assert_eq!(level_order(root), expected);
    }

    #[test]
    fn test_level_order_zig_zag_like_tree() {
        let root = create_tree_from_vec(&[Some(1), None, Some(2), Some(3), Some(4)]);
        let expected = vec![vec![1], vec![2], vec![3, 4]];
        assert_eq!(level_order(root), expected);
    }
}

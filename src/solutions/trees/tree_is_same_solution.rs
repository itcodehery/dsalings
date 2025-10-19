// solutions/trees/tree_is_same_solution.rs
// Implement the function to check if two binary trees are identical.

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

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p_node), Some(q_node)) => {
            let p_borrowed = p_node.borrow();
            let q_borrowed = q_node.borrow();

            p_borrowed.val == q_borrowed.val
                && is_same_tree(p_borrowed.left.clone(), q_borrowed.left.clone())
                && is_same_tree(p_borrowed.right.clone(), q_borrowed.right.clone())
        }
        _ => false,
    }
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
    fn test_is_same_tree_empty_trees() {
        let p = None;
        let q = None;
        assert!(is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_one_empty() {
        let p = create_tree_from_vec(&[Some(1)]);
        let q = None;
        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_example_1() {
        // p = [1,2,3], q = [1,2,3]
        let p = create_tree_from_vec(&[Some(1), Some(2), Some(3)]);
        let q = create_tree_from_vec(&[Some(1), Some(2), Some(3)]);
        assert!(is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_example_2() {
        // p = [1,2], q = [1,null,2]
        let p = create_tree_from_vec(&[Some(1), Some(2), None]);
        let q = create_tree_from_vec(&[Some(1), None, Some(2)]);
        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_different_values() {
        // p = [1,2,1], q = [1,2,2]
        let p = create_tree_from_vec(&[Some(1), Some(2), Some(1)]);
        let q = create_tree_from_vec(&[Some(1), Some(2), Some(2)]);
        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_different_structure() {
        // p = [1,null,2], q = [1,2,null]
        let p = create_tree_from_vec(&[Some(1), None, Some(2)]);
        let q = create_tree_from_vec(&[Some(1), Some(2), None]);
        assert!(!is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_complex_identical() {
        let p = create_tree_from_vec(&[
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(7),
            None,
            Some(18),
        ]);
        let q = create_tree_from_vec(&[
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(7),
            None,
            Some(18),
        ]);
        assert!(is_same_tree(p, q));
    }

    #[test]
    fn test_is_same_tree_complex_different() {
        let p = create_tree_from_vec(&[
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(7),
            None,
            Some(18),
        ]);
        let q = create_tree_from_vec(&[
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(8), // Different value\n            None,
            Some(18),
        ]);
        assert!(!is_same_tree(p, q));
    }
}

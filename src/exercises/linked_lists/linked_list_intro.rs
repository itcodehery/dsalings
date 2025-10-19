// exercises/linked_lists/linked_list_intro.rs
// Make me pass the test!
// Define the basic ListNode structure and implement its `new` function.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // Implement the 'new' function to create a new ListNode.
    // It should take an integer value and return a new ListNode with
    // its `val` set to the given value and `next` set to `None`.
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = ListNode::new(5);
        assert_eq!(node.val, 5);
        assert!(node.next.is_none());
    }

    #[test]
    fn test_new_node_zero() {
        let node = ListNode::new(0);
        assert_eq!(node.val, 0);
        assert!(node.next.is_none());
    }

    #[test]
    fn test_new_node_negative() {
        let node = ListNode::new(-10);
        assert_eq!(node.val, -10);
        assert!(node.next.is_none());
    }
}

// exercises/linked_lists/linked_list_length.rs
// Make me pass the test!
// Implement the `get_length` function to calculate the number of nodes in a singly linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Implement the `get_length` function.
// It should take a reference to the head of the list (Option<Box<ListNode>>).
// It should return the number of nodes in the list.
// An empty list should return 0.
pub fn get_length(head: &Option<Box<ListNode>>) -> usize {
    // Your code here
    0 // Placeholder, make this return the correct length
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_list(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = head;
            head = Some(new_node);
        }
        head
    }

    #[test]
    fn test_length_empty_list() {
        let head = None;
        assert_eq!(get_length(&head), 0);
    }

    #[test]
    fn test_length_single_node_list() {
        let head = create_list(&[1]);
        assert_eq!(get_length(&head), 1);
    }

    #[test]
    fn test_length_multiple_nodes_list() {
        let head = create_list(&[1, 2, 3, 4, 5]);
        assert_eq!(get_length(&head), 5);
    }

    #[test]
    fn test_length_two_nodes_list() {
        let head = create_list(&[10, 20]);
        assert_eq!(get_length(&head), 2);
    }

    #[test]
    fn test_length_longer_list() {
        let head = create_list(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(get_length(&head), 10);
    }
}

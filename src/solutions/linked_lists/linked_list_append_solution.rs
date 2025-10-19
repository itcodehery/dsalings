// solutions/linked_lists/linked_list_append_solution.rs
// Implement the `append` function to add a new node to the end of a singly linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> ListNode {
        ListNode { val, next: None }
    }
}

// Implement the `append` function.
// It should take a mutable reference to the head of the list (Option<Box<ListNode>>)
// and an integer value for the new node.
// It should append a new node with the given value to the end of the list.
// If the list is empty, the new node becomes the head.
pub fn append(head: &mut Option<Box<ListNode>>, val: i32) {
    let new_node = Some(Box::new(ListNode::new(val)));

    if head.is_none() {
        *head = new_node;
        return;
    }

    let mut current = head;
    while let Some(node) = current {
        if node.next.is_none() {
            node.next = new_node;
            return;
        }
        current = &mut node.next;
    }
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
    fn test_append_to_empty_list() {
        let mut head = None;
        append(&mut head, 1);
        let expected = create_list(&[1]);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_append_to_non_empty_list() {
        let mut head = create_list(&[1, 2]);
        append(&mut head, 3);
        let expected = create_list(&[1, 2, 3]);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_append_multiple_times() {
        let mut head = None;
        append(&mut head, 10);
        append(&mut head, 20);
        append(&mut head, 30);
        let expected = create_list(&[10, 20, 30]);
        assert_eq!(head, expected);
    }

    #[test]
    fn test_append_zero() {
        let mut head = create_list(&[5]);
        append(&mut head, 0);
        let expected = create_list(&[5, 0]);
        assert_eq!(head, expected);
    }
}

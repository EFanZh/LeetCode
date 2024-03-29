use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn remove_elements_helper(maybe_node: &mut Option<Box<ListNode>>, val: i32) {
        if let Some(node) = maybe_node.as_deref_mut() {
            if node.val == val {
                *maybe_node = node.next.take();

                Self::remove_elements_helper(maybe_node, val);
            } else {
                Self::remove_elements_helper(&mut node.next, val);
            }
        }
    }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;

        Self::remove_elements_helper(&mut head, val);

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        Self::remove_elements(head, val)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

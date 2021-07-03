use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn remove_elements_helper(mut maybe_node: &mut Option<Box<ListNode>>, val: i32) {
        while let Some(node) = maybe_node.as_deref_mut() {
            if node.val == val {
                *maybe_node = node.next.take();
            } else {
                maybe_node = &mut node.next;
            }
        }
    }

    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
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

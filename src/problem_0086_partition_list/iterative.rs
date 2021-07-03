use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left = None;
        let mut right = None;
        let mut left_tail = &mut left;
        let mut right_tail = &mut right;

        while let Some(mut node) = head {
            head = node.next.take();

            if node.val < x {
                left_tail = &mut left_tail.get_or_insert(node).next;
            } else {
                right_tail = &mut right_tail.get_or_insert(node).next;
            }
        }

        *left_tail = right;

        left
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        Self::partition(head, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

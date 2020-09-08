use super::super::data_structures::ListNode;

pub struct Solution;

impl Solution {
    fn partition_helper<'a>(
        left_tail: &'a mut Option<Box<ListNode>>,
        right_tail: &mut Option<Box<ListNode>>,
        head: Option<Box<ListNode>>,
        x: i32,
    ) -> &'a mut Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let next = node.next.take();

            if node.val < x {
                Self::partition_helper(&mut left_tail.get_or_insert(node).next, right_tail, next, x)
            } else {
                Self::partition_helper(left_tail, &mut right_tail.get_or_insert(node).next, next, x)
            }
        } else {
            left_tail
        }
    }

    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left = None;
        let mut right = None;
        let left_tail = Self::partition_helper(&mut left, &mut right, head, x);

        *left_tail = right;

        left
    }
}

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

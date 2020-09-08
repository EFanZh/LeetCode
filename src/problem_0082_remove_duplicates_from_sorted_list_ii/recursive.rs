use super::super::data_structures::ListNode;

pub struct Solution;

impl Solution {
    fn delete_duplicates_double(prev: i32, head: Option<Box<ListNode>>, target: &mut Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            if node.val == prev {
                Self::delete_duplicates_double(prev, node.next.take(), target);
            } else {
                let next = node.next.take();

                Self::delete_duplicates_single(node, next, target);
            }
        }
    }

    fn delete_duplicates_single(prev: Box<ListNode>, head: Option<Box<ListNode>>, target: &mut Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            if node.val == prev.val {
                Self::delete_duplicates_double(node.val, node.next.take(), target);
            } else {
                let next = node.next.take();

                Self::delete_duplicates_single(node, next, &mut target.get_or_insert(prev).next);
            }
        } else {
            *target = Some(prev);
        }
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        if let Some(mut head_node) = head {
            let next = head_node.next.take();

            Self::delete_duplicates_single(head_node, next, &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::delete_duplicates(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

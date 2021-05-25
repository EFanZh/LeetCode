use super::super::data_structures::ListNode;

pub struct Solution;

impl Solution {
    fn delete_duplicates_helper(tail: &mut ListNode, head: Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            if node.val == tail.val {
                Self::delete_duplicates_helper(tail, node.next);
            } else {
                let next = node.next.take();

                Self::delete_duplicates_helper(tail.next.get_or_insert(node), next);
            }
        }
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut node| {
            let next = node.next.take();

            Self::delete_duplicates_helper(node.as_mut(), next);

            node
        })
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

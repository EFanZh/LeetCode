use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn delete_duplicates_helper(tail: &mut ListNode, head: Option<Box<ListNode>>) {
        if let Some(mut node) = head {
            if node.val == tail.val {
                Self::delete_duplicates_helper(tail, node.next);
            } else {
                let next = node.next.take();

                Self::delete_duplicates_helper(tail.next.insert(node), next);
            }
        }
    }

    #[expect(clippy::single_option_map, reason = "required")]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut node| {
            let next = node.next.take();

            Self::delete_duplicates_helper(node.as_mut(), next);

            node
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

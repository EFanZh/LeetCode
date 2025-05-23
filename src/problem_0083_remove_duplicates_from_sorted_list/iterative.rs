use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::single_option_map, reason = "required")]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut node| {
            let mut tail = node.as_mut();
            let mut head = tail.next.take();

            while let Some(mut node) = head {
                head = node.next.take();

                if node.val != tail.val {
                    tail = tail.next.insert(node);
                }
            }

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

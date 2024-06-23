use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut iter = head;

        while let Some(mut node) = iter {
            let mut sum = node.val;

            loop {
                node = node.next.unwrap();

                if node.val == 0 {
                    iter = node.next.take();
                    node.val = sum;
                    tail = &mut tail.insert(node).next;

                    break;
                }

                sum += node.val;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_nodes(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;

        while let Some(mut node) = head {
            let mut target = &mut result;

            head = node.next.take();

            loop {
                if let Some(target_node) = target {
                    if target_node.val < node.val {
                        target = &mut target_node.next;
                    } else {
                        target_node.next = Some(mem::replace(target_node, node));

                        break;
                    }
                } else {
                    *target = Some(node);

                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::insertion_sort_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

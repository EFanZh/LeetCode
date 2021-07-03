use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;

        loop {
            match (&mut l1, &mut l2) {
                (None, None) => {
                    break;
                }
                (None, rest @ Some(_)) | (rest @ Some(_), None) => {
                    *target = rest.take();

                    break;
                }
                (Some(left), Some(right)) => {
                    *target = if left.val <= right.val {
                        let new_l1 = left.next.take();

                        mem::replace(&mut l1, new_l1)
                    } else {
                        let new_l2 = right.next.take();

                        mem::replace(&mut l2, new_l2)
                    };

                    target = &mut target.as_mut().unwrap().next;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::merge_two_lists(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

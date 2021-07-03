use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ptr::NonNull;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut scout = &head;

        for _ in 0..n {
            scout = &scout.as_ref().unwrap().next;
        }

        let mut scout = scout.as_deref().map(NonNull::from);
        let mut node = &mut head;

        while let Some(s) = scout {
            node = &mut node.as_mut().unwrap().next;
            scout = unsafe { s.as_ref().next.as_deref().map(NonNull::from) }; // `unsafe` is not recommended.
        }

        *node = node.as_mut().unwrap().next.take();

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end(head, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let m = m as u32 - 1;
        let n = n as u32;
        let mut rest = &mut head;
        let mut i = 0;

        // Left.

        while i < m {
            i += 1;
            rest = &mut rest.as_deref_mut().unwrap().next;
        }

        // Middle.

        let mut iter = rest.take();
        let mut middle = None;

        while i < n {
            i += 1;

            let mut node = iter.unwrap();

            iter = mem::replace(&mut node.next, middle);
            middle = Some(node);
        }

        drop(mem::replace(rest, middle));

        while let Some(node) = rest {
            rest = &mut node.next;
        }

        // Right.

        drop(mem::replace(rest, iter));

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        Self::reverse_between(head, m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

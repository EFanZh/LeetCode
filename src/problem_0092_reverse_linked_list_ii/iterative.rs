use super::super::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::ptr::NonNull;

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, mut m: i32, n: i32) -> Option<Box<ListNode>> {
        m -= 1;

        let length = n - m;

        if length > 1 {
            let mut tail = &mut head;

            for _ in 0..m {
                tail = &mut tail.as_mut().unwrap().next;
            }

            let mut reversed = tail.take();
            let reversed_next = &mut reversed.as_deref_mut().unwrap().next;
            let mut rest = reversed_next.take();
            let mut reversed_tail = NonNull::from(reversed_next);

            for _ in 1..length {
                let mut node = rest.unwrap();

                rest = mem::replace(&mut node.next, reversed);
                reversed = Some(node);
            }

            unsafe {
                *reversed_tail.as_mut() = rest;
            }

            *tail = reversed;
        }

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

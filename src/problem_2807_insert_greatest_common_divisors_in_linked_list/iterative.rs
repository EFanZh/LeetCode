use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = head;
        let mut cursor = &mut result;
        let first_node = cursor.as_deref_mut().unwrap();
        let mut prev_value = first_node.val as u32;

        cursor = &mut first_node.next;

        while let Some(node) = cursor.take() {
            let value = node.val as u32;
            let gcd = Self::gcd(mem::replace(&mut prev_value, value), value);

            cursor = &mut cursor
                .insert(Box::new(ListNode {
                    val: gcd as _,
                    next: Some(node),
                }))
                .next
                .as_deref_mut()
                .unwrap()
                .next;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::insert_greatest_common_divisors(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

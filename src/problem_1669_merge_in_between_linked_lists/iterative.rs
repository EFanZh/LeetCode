use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let a = a as u32;
        let b = b as u32;

        // Find the node of index `a`.

        let mut cursor = &mut list1;

        for _ in 0..a {
            cursor = &mut cursor.as_deref_mut().unwrap().next;
        }

        // Append `list2` at index `a`.

        let mut list_1_rest = mem::replace(cursor, list2);

        // Remove unused nodes.

        for _ in a..=b {
            list_1_rest = list_1_rest.unwrap().next;
        }

        // Find the tail of the new list.

        while let Some(node) = cursor {
            cursor = &mut node.next;
        }

        // Append remaining nodes to the new list.

        *cursor = list_1_rest;

        list1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::merge_in_between(list1, a, b, list2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

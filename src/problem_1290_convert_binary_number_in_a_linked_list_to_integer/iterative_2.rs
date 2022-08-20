use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut maybe_node = head;
        let mut result = 0;

        while let Some(node) = maybe_node {
            result = result * 2 + node.val;
            maybe_node = node.next;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        Self::get_decimal_value(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

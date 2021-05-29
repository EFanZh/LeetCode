use super::super::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = l1;
        let mut lhs = &mut result;
        let mut rhs = l2;
        let mut carry = 0;

        loop {
            if let Some(lhs_node) = lhs {
                if let Some(rhs_node) = rhs {
                    lhs_node.val += rhs_node.val + carry;
                    carry = lhs_node.val / 10;
                    lhs_node.val %= 10;

                    lhs = &mut lhs_node.next;
                    rhs = rhs_node.next;
                } else {
                    lhs_node.val += carry;
                    carry = lhs_node.val / 10;
                    lhs_node.val %= 10;

                    lhs = &mut lhs_node.next;
                }
            } else if let Some(mut rhs_node) = rhs {
                rhs_node.val += carry;
                carry = rhs_node.val / 10;
                rhs_node.val %= 10;

                rhs = rhs_node.next.take();
                *lhs = Some(rhs_node);
                lhs = &mut lhs.as_mut().unwrap().next;
            } else {
                if carry != 0 {
                    *lhs = Some(Box::new(ListNode::new(carry)));
                }

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two_numbers(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

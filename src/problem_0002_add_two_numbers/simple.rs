use super::super::data_structures::ListNode;

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = l1;
        let mut lhs = &mut result;
        let mut rhs = l2;
        let mut carry = false;

        loop {
            if let Some(lhs_node) = lhs {
                if let Some(rhs_node) = rhs {
                    lhs_node.val += rhs_node.val + (carry as i32);
                    carry = lhs_node.val >= 10;
                    lhs_node.val %= 10;

                    lhs = &mut lhs_node.next;
                    rhs = rhs_node.next;
                } else {
                    lhs_node.val += carry as i32;
                    carry = lhs_node.val >= 10;
                    lhs_node.val %= 10;

                    lhs = &mut lhs_node.next;
                }
            } else if let Some(mut rhs_node) = rhs {
                rhs_node.val += carry as i32;
                carry = rhs_node.val >= 10;
                rhs_node.val %= 10;

                rhs = rhs_node.next.take();
                *lhs = Some(rhs_node);
                lhs = &mut lhs.as_mut().unwrap().next;
            } else {
                if carry {
                    *lhs = Some(Box::new(ListNode::new(1)));
                }

                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two_numbers(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}

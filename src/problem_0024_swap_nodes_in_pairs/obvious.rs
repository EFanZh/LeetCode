use super::super::data_structures::ListNode;

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;

        while let Some(mut node) = head {
            if let Some(mut next) = node.next.take() {
                head = next.next.take();
                target = &mut target.get_or_insert(next).next.get_or_insert(node).next;
            } else {
                *target = Some(node);

                break;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::swap_pairs(head)
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

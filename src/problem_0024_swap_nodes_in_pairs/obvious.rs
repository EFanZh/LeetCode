use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;
        let mut head = head;

        while let Some(mut node) = head {
            if let Some(mut next) = node.next.take() {
                head = next.next.take();
                target = &mut target.insert(next).next.insert(node).next;
            } else {
                *target = Some(node);

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::swap_pairs(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

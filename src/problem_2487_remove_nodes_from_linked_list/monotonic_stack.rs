use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = None::<Box<ListNode>>;
        let mut iter = head;

        while let Some(mut node) = iter {
            iter = node.next.take();

            while let Some(top) = &stack {
                if top.val < node.val {
                    stack = stack.unwrap().next;
                } else {
                    break;
                }
            }

            node.next = stack;
            stack = Some(node);
        }

        let mut result = None;
        let mut iter = stack;

        while let Some(mut node) = iter {
            iter = node.next.take();
            node.next = result;
            result = Some(node);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::remove_nodes(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

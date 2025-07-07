use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::unnecessary_box_returns, reason = "false positive")]
    fn helper(mut head: Box<ListNode>) -> Box<ListNode> {
        let mut node = head.as_mut();

        node.val *= 2;

        if node.val >= 10 {
            node.val -= 10;

            head = Box::new(ListNode {
                val: 1,
                next: Some(head),
            });

            node = head.next.as_deref_mut().unwrap();
        }

        while let Some(next) = node.next.as_deref_mut() {
            next.val *= 2;

            if next.val >= 10 {
                next.val -= 10;
                node.val += 1;
            }

            node = next;
        }

        head
    }

    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(Self::helper)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::double_it(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

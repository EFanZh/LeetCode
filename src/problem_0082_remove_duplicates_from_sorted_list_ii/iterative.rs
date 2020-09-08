use super::super::data_structures::ListNode;

pub struct Solution;

impl Solution {
    fn delete_duplicates_helper(
        mut prev: Box<ListNode>,
        mut head: Option<Box<ListNode>>,
        mut target: &mut Option<Box<ListNode>>,
    ) {
        loop {
            if let Some(mut node) = head {
                head = node.next.take();

                if node.val == prev.val {
                    loop {
                        if let Some(mut node) = head {
                            head = node.next.take();

                            if node.val != prev.val {
                                prev = node;

                                break;
                            }
                        } else {
                            return;
                        }
                    }
                } else {
                    target = &mut target.get_or_insert(prev).next;
                    prev = node;
                }
            } else {
                *target = Some(prev);

                break;
            }
        }
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        if let Some(mut head_node) = head {
            let next = head_node.next.take();

            Self::delete_duplicates_helper(head_node, next, &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::delete_duplicates(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

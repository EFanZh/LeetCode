use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;

        while let Some(mut node) = head {
            head = mem::replace(&mut node.next, result);
            result = Some(node);
        }

        result
    }

    fn append(mut tail: &mut Option<Box<ListNode>>, mut head: Option<Box<ListNode>>) -> &mut Option<Box<ListNode>> {
        while let Some(mut node) = head {
            head = node.next.take();
            tail = &mut tail.insert(node).next;
        }

        tail
    }

    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut result = None;
        let mut tail = &mut result;
        let mut group_length = 0_u32;
        let mut group_head;
        let mut group_tail;
        let mut required;

        'outer: loop {
            // Odd group.

            group_length += 1;
            group_head = None;
            group_tail = &mut group_head;
            required = group_length;

            loop {
                if let Some(mut node) = head {
                    head = node.next.take();
                    group_tail = &mut group_tail.insert(node).next;
                    required -= 1;

                    if required == 0 {
                        tail = Self::append(tail, group_head);

                        break;
                    }
                } else {
                    break 'outer;
                }
            }

            // Even group.

            group_length += 1;
            group_head = None;
            required = group_length;

            loop {
                if let Some(mut node) = head {
                    head = mem::replace(&mut node.next, group_head);
                    group_head = Some(node);
                    required -= 1;

                    if required == 0 {
                        tail = Self::append(tail, group_head);

                        break;
                    }
                } else {
                    break 'outer;
                }
            }
        }

        // Last group.

        if required % 2 != 0 {
            group_head = Self::reverse(group_head);
        }

        Self::append(tail, group_head);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse_even_length_groups(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

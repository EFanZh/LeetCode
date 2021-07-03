use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn split_at(mut head: Option<Box<ListNode>>, n: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut cursor = &mut head;

        for _ in 0..n {
            cursor = &mut cursor.as_deref_mut().unwrap().next;
        }

        let rest = cursor.take();

        (head, rest)
    }

    fn merge(
        group_1: Option<Box<ListNode>>,
        group_2: Option<Box<ListNode>>,
        mut target: &mut Option<Box<ListNode>>,
    ) -> &mut Option<Box<ListNode>> {
        match (group_1, group_2) {
            (None, None) => return target,
            (None, Some(group)) | (Some(group), None) => target = &mut target.get_or_insert(group).next,
            (Some(mut left), Some(mut right)) => loop {
                if right.val < left.val {
                    target = &mut target.get_or_insert(right).next;

                    if let Some(next_right) = target.take() {
                        right = next_right;
                    } else {
                        target = &mut target.get_or_insert(left).next;

                        break;
                    }
                } else {
                    target = &mut target.get_or_insert(left).next;

                    if let Some(next_left) = target.take() {
                        left = next_left;
                    } else {
                        target = &mut target.get_or_insert(right).next;

                        break;
                    }
                }
            },
        };

        while let Some(node) = target {
            target = &mut node.next;
        }

        target
    }

    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let mut group_length = 1;

        while group_length < length {
            let next_group_length = group_length + group_length;
            let mut next_head = None;
            let mut tail = &mut next_head;

            for _ in 0..length / next_group_length {
                let (group_1, rest) = Self::split_at(head, group_length);
                let (group_2, rest) = Self::split_at(rest, group_length);

                tail = Self::merge(group_1, group_2, tail);
                head = rest;
            }

            if length % next_group_length <= group_length {
                *tail = head;
            } else {
                let (group_1, group_2) = Self::split_at(head, group_length);

                Self::merge(group_1, group_2, tail);
            }

            head = next_head;
            group_length = next_group_length;
        }

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::sort_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

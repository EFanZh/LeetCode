use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::iter;

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let first_node = *head.unwrap();
        let second_node = *first_node.next.unwrap();
        let mut prev_1 = first_node.val;
        let mut prev_2 = second_node.val;

        let mut iter = iter::successors(second_node.next.as_deref(), |node| node.next.as_deref())
            .enumerate()
            .filter_map(|(i, node)| {
                let result = (match prev_1.cmp(&prev_2) {
                    Ordering::Less => node.val < prev_2,
                    Ordering::Equal => false,
                    Ordering::Greater => prev_2 < node.val,
                })
                .then_some(i);

                prev_1 = prev_2;
                prev_2 = node.val;

                result
            });

        iter.next()
            .and_then(|first| Some((first, iter.next()?)))
            .map_or([-1, -1], |(first, second)| {
                let mut min_distance = second - first;
                let mut prev = second;

                for i in iter {
                    min_distance = min_distance.min(i - prev);
                    prev = i;
                }

                [min_distance as _, (prev - first) as _]
            })
            .into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        Self::nodes_between_critical_points(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

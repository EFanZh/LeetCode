pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::{BinaryHeap, VecDeque};
use std::{convert, mem};

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let costs = costs.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let n = costs.len();
        let k = k as u32 as usize;
        let candidates = candidates as u32 as usize;

        if k >= n {
            return costs.iter().fold(0, |sum, &cost| sum + i64::from(cost));
        }

        if n <= candidates * 2 {
            let mut costs = (0_u32..).zip(costs).map(|(i, cost)| (cost, i)).collect::<Box<_>>();

            costs.select_nth_unstable(k - 1);

            return costs[..k].iter().fold(0, |sum, &(cost, _)| sum + i64::from(cost));
        }

        let mut queue = VecDeque::from(costs);
        let mut right_heap = queue.drain(n - candidates..).map(Reverse).collect::<BinaryHeap<_>>();
        let mut left_heap = queue.drain(..candidates).map(Reverse).collect::<BinaryHeap<_>>();
        let mut result = 0;

        for _ in 0..k {
            let left = left_heap.peek_mut();
            let right = right_heap.peek_mut();

            let (selected, ignored, replace) = if let Some(left) = left {
                if let Some(right) = right {
                    if right.0 < left.0 {
                        (right, Some(left), queue.pop_back())
                    } else {
                        (left, Some(right), queue.pop_front())
                    }
                } else {
                    (left, None, queue.pop_front())
                }
            } else {
                (right.unwrap(), None, queue.pop_back())
            };

            drop(ignored);

            result += i64::from(if let Some(new_value) = replace {
                mem::replace(&mut convert::identity(selected).0, new_value)
            } else {
                PeekMut::pop(selected).0
            });
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        Self::total_cost(costs, k, candidates)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

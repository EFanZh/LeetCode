pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k as u32;
        let mut counts = HashMap::new();

        for value in arr {
            counts
                .entry(value)
                .and_modify(|count: &mut u32| *count += 1)
                .or_insert(1);
        }

        let mut queue = counts.into_values().map(Reverse).collect::<BinaryHeap<_>>();

        while k != 0 {
            let top = queue.peek_mut().unwrap();

            if let Some(new_k) = k.checked_sub(top.0) {
                PeekMut::pop(top);
                k = new_k;
            } else {
                break;
            }
        }

        queue.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        Self::find_least_num_of_unique_ints(arr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut rows = (0_u32..)
            .zip(mat)
            .map(|(i, row)| Reverse((row.into_iter().sum::<i32>() as u32, i)))
            .collect::<BinaryHeap<_>>();

        let mut result = Vec::with_capacity(k);

        result.extend(iter::from_fn(|| rows.pop().map(|Reverse((_, i))| i as i32)).take(k));

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        Self::k_weakest_rows(mat, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let mut start = matrix[0][0];
        let mut end = matrix[n - 1][n - 1];

        while start != end {
            let middle = start + (end - start) / 2;

            let rank = matrix
                .iter()
                .map(|row| {
                    row.binary_search_by(|&v| if v <= middle { Ordering::Less } else { Ordering::Greater })
                        .unwrap_err() as i32
                })
                .sum::<i32>();

            if rank < k {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        start
    }
}

impl super::Solution for Solution {
    fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::kth_smallest(matrix, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

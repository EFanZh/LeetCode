pub struct Solution;

use std::mem;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let columns = matrix.first().map_or(0, Vec::len) as i32;
        let mut cache = vec![(0, 0); columns as _];
        let mut result = 0;

        for row in matrix {
            let mut prev_max_square = 0;
            let mut max_left = 0;

            for ((max_square, max_height), c) in cache.iter_mut().zip(&row) {
                if *c == '0' {
                    max_left = 0;
                    *max_height = 0;
                    prev_max_square = mem::replace(max_square, 0);
                } else {
                    max_left += 1;
                    *max_height += 1;
                    prev_max_square = mem::replace(max_square, max_left.min(*max_height).min(prev_max_square + 1));

                    result = result.max(*max_square);
                }
            }
        }

        result * result
    }
}

impl super::Solution for Solution {
    fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        Self::maximal_square(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

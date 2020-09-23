pub struct Solution;

use std::mem;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let columns = matrix.first().map_or(0, Vec::len) as i32;
        let mut cache = vec![0; columns as _];
        let mut result = 0;

        for row in matrix {
            let mut prev_max_square_1 = 0;
            let mut prev_max_square_2 = 0;

            for (max_square, c) in cache.iter_mut().zip(&row) {
                if *c == '0' {
                    prev_max_square_2 = 0;
                    prev_max_square_1 = mem::replace(max_square, prev_max_square_2);
                } else {
                    prev_max_square_2 = prev_max_square_1.min(prev_max_square_2).min(*max_square) + 1;
                    prev_max_square_1 = mem::replace(max_square, prev_max_square_2);

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

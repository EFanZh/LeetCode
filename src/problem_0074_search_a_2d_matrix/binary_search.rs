pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let columns = matrix.first().map_or(0, Vec::len);
        let mut start = 0;
        let mut length = columns * matrix.len();

        while length > 0 {
            let half = length / 2;
            let middle = start + half;

            match target.cmp(&matrix[middle / columns][middle % columns]) {
                Ordering::Less => length = half,
                Ordering::Equal => return true,
                Ordering::Greater => {
                    start = middle + 1;
                    length -= half + 1;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Self::search_matrix(matrix, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

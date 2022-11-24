pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let columns = matrix.first().map_or(0, Vec::len);
        let mut cache = vec![0_u32; columns];
        let mut result = 0;

        for row in matrix {
            let mut top_left = 0;
            let mut left = 0;

            for (top, value) in cache.iter_mut().zip(row) {
                if value == 0 {
                    *top = 0;
                } else {
                    let count = top_left.min(*top).min(left) + 1;

                    result += count;

                    top_left = *top;
                    *top = count;
                }

                left = *top;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        Self::count_squares(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

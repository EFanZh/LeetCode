pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut min_sum_1 = 0;
        let mut min_sum_1_column = n;
        let mut min_sum_2 = 0;

        for row in grid {
            let mut new_min_sum_1 = i32::MAX;
            let mut new_min_sum_1_column = n;
            let mut new_min_sum_2 = i32::MAX;

            for (i, value) in row.into_iter().enumerate() {
                let sum = value + if i == min_sum_1_column { min_sum_2 } else { min_sum_1 };

                if sum < new_min_sum_1 {
                    new_min_sum_2 = new_min_sum_1;
                    new_min_sum_1 = sum;
                    new_min_sum_1_column = i;
                } else if sum < new_min_sum_2 {
                    new_min_sum_2 = sum;
                }
            }

            min_sum_1 = new_min_sum_1;
            min_sum_1_column = new_min_sum_1_column;
            min_sum_2 = new_min_sum_2;
        }

        min_sum_1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_falling_path_sum(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

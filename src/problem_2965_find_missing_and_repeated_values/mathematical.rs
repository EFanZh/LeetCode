pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = (grid.len() as u32).pow(2);
        let mut sum = 0;
        let mut square_sum = 0;

        grid.into_iter().flatten().for_each(|value| {
            sum += value;
            square_sum += i64::from(value.cast_unsigned().pow(2));
        });

        let expected_sum = n * (1 + n) / 2;

        let expected_square_sum = {
            let n = u64::from(n);

            n * (1 + n) * (2 * n + 1) / 6
        };

        let a_minus_b = sum - expected_sum.cast_signed();
        let a_squared_minus_b_squared = (square_sum - expected_square_sum.cast_signed()) as i32;
        let a_plus_b = a_squared_minus_b_squared / a_minus_b;
        let a = (a_minus_b + a_plus_b) >> 1;
        let b = a_plus_b - a;

        vec![a, b]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_missing_and_repeated_values(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

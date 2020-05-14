pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut result = vec![vec![0; n]; n];
        let mut previous_value = 0;

        let mut next_value = move || {
            previous_value += 1;

            previous_value
        };

        for i in 0..n / 2 {
            // Right.

            for target in &mut result[i][i..n - i] {
                *target = next_value();
            }

            // Down.

            for row in &mut result[i + 1..n - i - 1] {
                row[n - i - 1] = next_value();
            }

            // Left.

            for target in result[n - i - 1][i..n - i].iter_mut().rev() {
                *target = next_value();
            }

            // Up.

            for row in result[i + 1..n - i - 1].iter_mut().rev() {
                row[i] = next_value();
            }
        }

        if n % 2 == 1 {
            result[n / 2][n / 2] = next_value();
        }

        result
    }
}

impl super::Solution for Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        Self::generate_matrix(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

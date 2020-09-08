pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut result = Vec::with_capacity(num_rows);

        if num_rows > 0 {
            result.push(vec![1]);

            for length in 2..=num_rows {
                let mut row = Vec::with_capacity(length);
                let previous_row = result.last().unwrap().as_slice();

                row.push(1);
                row.extend(previous_row.iter().zip(&previous_row[1..]).map(|(lhs, rhs)| lhs + rhs));
                row.push(1);

                result.push(row);
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        Self::generate(num_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

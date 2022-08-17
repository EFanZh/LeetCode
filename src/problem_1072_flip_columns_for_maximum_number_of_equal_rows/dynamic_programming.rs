pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut buckets = HashMap::new();
        let mut result = 0;

        for row in &matrix {
            let (&first, rest) = row.split_first().unwrap();
            let mut key = [0_u8; 38];

            for (i, &value) in rest.iter().enumerate() {
                if value != first {
                    key[i / 8] |= 1 << (i % 8);
                }
            }

            result = result.max(*buckets.entry(key).and_modify(|count| *count += 1).or_insert(1));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        Self::max_equal_rows_after_flips(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

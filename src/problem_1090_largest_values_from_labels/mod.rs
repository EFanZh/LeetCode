pub mod greedy;

pub trait Solution {
    fn largest_vals_from_labels(values: Vec<i32>, labels: Vec<i32>, num_wanted: i32, use_limit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 4, 3, 2, 1] as &[_], &[1, 1, 2, 2, 3] as &[_], 3, 1), 9),
            ((&[5, 4, 3, 2, 1], &[1, 3, 3, 3, 2], 3, 2), 12),
            ((&[9, 8, 8, 7, 6], &[0, 0, 0, 1, 1], 3, 1), 16),
        ];

        for ((values, labels, num_wanted, use_limit), expected) in test_cases {
            assert_eq!(
                S::largest_vals_from_labels(values.to_vec(), labels.to_vec(), num_wanted, use_limit),
                expected,
            );
        }
    }
}

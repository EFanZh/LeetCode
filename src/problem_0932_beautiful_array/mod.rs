pub mod dynamic_programming;

pub trait Solution {
    fn beautiful_array(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 100];
        let mut right_set = Vec::new();

        for n in test_cases {
            let mut result = S::beautiful_array(n);

            // Check result size.

            assert_eq!(result.len(), n as usize);

            // Check beautiful array property.

            right_set.resize(n as _, true);

            for (i, &middle) in result.iter().enumerate() {
                right_set[middle as usize - 1] = false;

                for &left in &result[..i] {
                    let right = middle * 2 - left;

                    assert_ne!(right_set.get(right as usize - 1).copied(), Some(true));
                }
            }

            right_set.clear();

            // Check permutation.

            result.sort_unstable();

            assert!((1..).zip(result).all(|(expected, actual)| expected == actual));
        }
    }
}

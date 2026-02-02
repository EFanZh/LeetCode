pub mod dynamic_programming;

pub trait Solution {
    fn max_removals(source: String, pattern: String, target_indices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abbaa", "aba", &[0, 1, 2] as &[_]), 1),
            (("bcda", "d", &[0, 3]), 2),
            (("dda", "dda", &[0, 1, 2]), 0),
            (("yeyeykyded", "yeyyd", &[0, 2, 3, 4]), 2),
        ];

        for ((source, pattern, target_indices), expected) in test_cases {
            assert_eq!(
                S::max_removals(source.to_string(), pattern.to_string(), target_indices.to_vec()),
                expected,
            );
        }
    }
}

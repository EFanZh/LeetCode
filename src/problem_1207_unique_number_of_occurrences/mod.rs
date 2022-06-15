pub mod hash_map_and_hash_set;

pub trait Solution {
    fn unique_occurrences(arr: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 2, 1, 1, 3] as &[_], true),
            (&[1, 2], false),
            (&[-3, 0, 1, -3, 1, 1, 1, -3, 10, 0], true),
        ];

        for (arr, expected) in test_cases {
            assert_eq!(S::unique_occurrences(arr.to_vec()), expected);
        }
    }
}

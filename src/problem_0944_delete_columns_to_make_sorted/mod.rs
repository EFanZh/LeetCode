pub mod brute_force;

pub trait Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["cba", "daf", "ghi"] as &[_], 1),
            (&["a", "b"], 0),
            (&["zyx", "wvu", "tsr"], 3),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::min_deletion_size(strs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}

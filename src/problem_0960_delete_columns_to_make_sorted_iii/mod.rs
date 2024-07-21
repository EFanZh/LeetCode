pub mod dynamic_programming;

pub trait Solution {
    fn min_deletion_size(strs: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["babca", "bbazb"] as &[_], 3),
            (&["edcba"], 4),
            (&["ghi", "def", "abc"], 0),
        ];

        for (strs, expected) in test_cases {
            assert_eq!(
                S::min_deletion_size(strs.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}

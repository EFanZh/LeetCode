pub mod sliding_window;

pub trait Solution {
    fn count_k_constraint_substrings(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("10101", 1), 12),
            (("1010101", 2), 25),
            (("11111", 1), 15),
            (("000011", 1), 18),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::count_k_constraint_substrings(s.to_string(), k), expected);
        }
    }
}

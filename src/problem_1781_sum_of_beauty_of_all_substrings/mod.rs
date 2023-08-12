pub mod brute_force;

pub trait Solution {
    fn beauty_sum(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aabcb", 5), ("aabcbaa", 17)];

        for (s, expected) in test_cases {
            assert_eq!(S::beauty_sum(s.to_string()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn count_substrings(s: String, t: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aba", "baba"), 6), (("ab", "bb"), 3)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::count_substrings(s.to_string(), t.to_string()), expected);
        }
    }
}

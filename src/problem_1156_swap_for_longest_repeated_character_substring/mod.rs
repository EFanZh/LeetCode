pub mod iterative;

pub trait Solution {
    fn max_rep_opt1(text: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ababa", 3), ("aaabaaa", 6), ("aaaaa", 5), ("bbababaaaa", 6)];

        for (text, expected) in test_cases {
            assert_eq!(S::max_rep_opt1(text.to_string()), expected);
        }
    }
}

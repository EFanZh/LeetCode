pub mod iterative;

pub trait Solution {
    fn score_of_string(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("hello", 13), ("zaz", 50)];

        for (s, expected) in test_cases {
            assert_eq!(S::score_of_string(s.to_string()), expected);
        }
    }
}

pub mod stack;

pub trait Solution {
    fn min_add_to_make_valid(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("())", 1), ("(((", 3)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_add_to_make_valid(s.to_string()), expected);
        }
    }
}

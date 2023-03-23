pub mod stack;

pub trait Solution {
    fn min_insertions(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("(()))", 1), ("())", 0), ("))())(", 3), ("()()()()()(", 7)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_insertions(s.to_string()), expected);
        }
    }
}

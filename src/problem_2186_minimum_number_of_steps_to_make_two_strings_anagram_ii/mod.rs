pub mod iterative;

pub trait Solution {
    fn min_steps(s: String, t: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("leetcode", "coats"), 7), (("night", "thing"), 0)];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::min_steps(s.to_string(), t.to_string()), expected);
        }
    }
}

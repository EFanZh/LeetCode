pub mod stack;

pub trait Solution {
    fn make_good(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leEeetcode", "leetcode"), ("abBAcC", ""), ("s", "s")];

        for (s, expected) in test_cases {
            assert_eq!(S::make_good(s.to_string()), expected);
        }
    }
}

pub mod greedy;

pub trait Solution {
    fn is_subsequence(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abc", "ahbgdc"), true), (("axc", "ahbgdc"), false)];

        for ((s, t), expected) in test_cases.iter().copied() {
            assert_eq!(S::is_subsequence(s.to_string(), t.to_string()), expected);
        }
    }
}

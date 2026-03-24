pub mod iterative;

pub trait Solution {
    fn has_special_substring(s: String, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aaabaaa", 3), true), (("abc", 2), false)];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::has_special_substring(s.to_string(), k), expected);
        }
    }
}

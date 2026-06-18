pub mod iterative;

pub trait Solution {
    fn reverse_prefix(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcd", 2), "bacd"), (("xyz", 3), "zyx"), (("hey", 1), "hey")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::reverse_prefix(s.to_string(), k), expected);
        }
    }
}

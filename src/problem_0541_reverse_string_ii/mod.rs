pub mod reverse_by_chunks;

pub trait Solution {
    fn reverse_str(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcdefg", 2), "bacdfeg"), (("abcd", 2), "bacd")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::reverse_str(s.to_string(), k), expected);
        }
    }
}

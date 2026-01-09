pub mod iterative;

pub trait Solution {
    fn string_hash(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("abcd", 2), "bf"), (("mxz", 3), "i")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::string_hash(s.to_string(), k), expected);
        }
    }
}

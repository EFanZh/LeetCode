pub mod iterative;

pub trait Solution {
    fn residue_prefixes(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abc", 2), ("dd", 1), ("bob", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::residue_prefixes(s.to_string()), expected);
        }
    }
}

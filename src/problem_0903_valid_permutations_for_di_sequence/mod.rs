pub mod dynamic_programming;

pub trait Solution {
    fn num_perms_di_sequence(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("DID", 5), ("D", 1), ("IDDDIIDIIIIIIIIDIDID", 853_197_538)];

        for (s, expected) in test_cases {
            assert_eq!(S::num_perms_di_sequence(s.to_string()), expected);
        }
    }
}

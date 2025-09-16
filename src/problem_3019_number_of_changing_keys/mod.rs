pub mod iterative;

pub trait Solution {
    fn count_key_changes(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aAbBcC", 2), ("AaAaAaaA", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_key_changes(s.to_string()), expected);
        }
    }
}

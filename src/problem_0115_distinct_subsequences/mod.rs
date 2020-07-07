pub mod dynamic_programming;

pub trait Solution {
    fn num_distinct(s: String, t: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("rabbbit", "rabbit"), 3), (("babgbag", "bag"), 5)];

        for ((s, t), expected) in test_cases.iter().copied() {
            assert_eq!(S::num_distinct(s.to_string(), t.to_string()), expected);
        }
    }
}

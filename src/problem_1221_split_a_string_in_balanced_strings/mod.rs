pub mod greedy;

pub trait Solution {
    fn balanced_string_split(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("RLRRLLRLRL", 4), ("RLLLLRRRLR", 3), ("LLLLRRRR", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::balanced_string_split(s.to_string()), expected);
        }
    }
}

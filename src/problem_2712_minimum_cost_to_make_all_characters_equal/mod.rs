pub mod greedy;

pub trait Solution {
    fn minimum_cost(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("0011", 2), ("010101", 9)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_cost(s.to_string()), expected);
        }
    }
}

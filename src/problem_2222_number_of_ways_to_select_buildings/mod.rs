pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn number_of_ways(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("001101", 6), ("11100", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::number_of_ways(s.to_string()), expected);
        }
    }
}

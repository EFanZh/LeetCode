pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn minimum_time(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1100101", 5), ("0010", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_time(s.to_string()), expected);
        }
    }
}

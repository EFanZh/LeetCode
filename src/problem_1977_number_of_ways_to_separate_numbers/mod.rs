pub mod dynamic_programming;

pub trait Solution {
    fn number_of_combinations(num: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("327", 2), ("094", 0), ("0", 0), ("99999", 7), ("9999999999999", 101)];

        for (num, expected) in test_cases {
            assert_eq!(S::number_of_combinations(num.to_string()), expected);
        }
    }
}

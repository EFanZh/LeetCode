pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn number_of_unique_good_subsequences(binary: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("001", 2), ("11", 2), ("101", 5), ("00011", 3), ("1110001", 23)];

        for (binary, expected) in test_cases {
            assert_eq!(S::number_of_unique_good_subsequences(binary.to_string()), expected);
        }
    }
}

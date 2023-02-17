pub mod dynamic_programming;

pub trait Solution {
    fn number_of_arrays(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1000", 10000), 1),
            (("1000", 10), 0),
            (("1317", 2000), 8),
            (("123454321", 3), 0),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::number_of_arrays(s.to_string(), k), expected);
        }
    }
}

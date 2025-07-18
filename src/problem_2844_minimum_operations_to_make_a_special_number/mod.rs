pub mod greedy;

pub trait Solution {
    fn minimum_operations(num: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("2245047", 2), ("2908305", 3), ("10", 1)];

        for (num, expected) in test_cases {
            assert_eq!(S::minimum_operations(num.to_string()), expected);
        }
    }
}

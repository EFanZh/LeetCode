pub mod greedy;

pub trait Solution {
    fn find_min_fibonacci_numbers(k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(7, 2), (10, 2), (19, 3), (645_157_245, 13)];

        for (k, expected) in test_cases {
            assert_eq!(S::find_min_fibonacci_numbers(k), expected);
        }
    }
}

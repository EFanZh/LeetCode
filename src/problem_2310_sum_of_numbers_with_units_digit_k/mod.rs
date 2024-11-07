pub mod iterative;

pub trait Solution {
    fn minimum_numbers(num: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((58, 9), 2), ((37, 2), -1), ((0, 7), 0), ((10, 8), -1), ((10, 5), 2)];

        for ((num, k), expected) in test_cases {
            assert_eq!(S::minimum_numbers(num, k), expected);
        }
    }
}

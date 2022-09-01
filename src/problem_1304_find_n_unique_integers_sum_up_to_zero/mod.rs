pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn sum_zero(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut buffer = HashSet::new();

        for n in test_cases {
            let mut sum = 0;

            for value in S::sum_zero(n) {
                sum += value;
                buffer.insert(value);
            }

            assert_eq!(sum, 0);
            assert_eq!(buffer.len(), n as _);

            buffer.clear();
        }
    }
}

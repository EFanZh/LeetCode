pub mod exponential_check;

pub trait Solution {
    fn divide(dividend: i32, divisor: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((10, 3), 3),
            ((7, -3), -2),
            ((1_100_540_749, -1_090_366_779), -1),
            ((2_147_483_647, 1), 2_147_483_647),
        ];

        for ((dividend, divisor), expected) in test_cases.iter().copied() {
            assert_eq!(S::divide(dividend, divisor), expected);
        }
    }
}

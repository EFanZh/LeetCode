pub mod binary_search;
pub mod iterative;

pub trait Solution {
    fn sequential_digits(low: i32, high: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((100, 300), &[123, 234] as &[_]),
            ((1_000, 13000), &[1_234, 2_345, 3_456, 4_567, 5_678, 6_789, 12345]),
            (
                (10, 1_000_000_000),
                &[
                    12,
                    23,
                    34,
                    45,
                    56,
                    67,
                    78,
                    89,
                    123,
                    234,
                    345,
                    456,
                    567,
                    678,
                    789,
                    1_234,
                    2_345,
                    3_456,
                    4_567,
                    5_678,
                    6_789,
                    12_345,
                    23_456,
                    34_567,
                    45_678,
                    56_789,
                    123_456,
                    234_567,
                    345_678,
                    456_789,
                    1_234_567,
                    2_345_678,
                    3_456_789,
                    12_345_678,
                    23_456_789,
                    123_456_789,
                ],
            ),
        ];

        for ((low, high), expected) in test_cases {
            assert_eq!(S::sequential_digits(low, high), expected);
        }
    }
}

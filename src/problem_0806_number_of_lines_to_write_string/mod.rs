pub mod iterative;

pub trait Solution {
    fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    [
                        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                        10, 10,
                    ],
                    "abcdefghijklmnopqrstuvwxyz",
                ),
                [3, 60],
            ),
            (
                (
                    [
                        4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                        10, 10,
                    ],
                    "bbbcccdddaaa",
                ),
                [2, 4],
            ),
        ];

        for ((widths, s), expected) in test_cases {
            assert_eq!(S::number_of_lines(widths.into(), s.into()), expected);
        }
    }
}

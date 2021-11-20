pub mod iterative;

pub trait Solution {
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((12, &[10, 8, 0, 5, 3] as &[_], &[2, 4, 1, 1, 3] as &[_]), 3),
            ((10, &[3], &[3]), 1),
            ((100, &[0, 2, 4], &[4, 2, 1]), 1),
            (
                (
                    660_732,
                    &[
                        620_831, 145_366, 229_113, 144_305, 382_893, 514_856, 171_642, 87_230, 409_014, 65_613,
                    ],
                    &[
                        327_716, 69_772, 667_805, 856_849, 78_755, 606_982, 696_937, 207_697, 275_337, 290_550,
                    ],
                ),
                5,
            ),
        ];

        for ((target, position, speed), expected) in test_cases {
            assert_eq!(S::car_fleet(target, position.to_vec(), speed.to_vec()), expected);
        }
    }
}

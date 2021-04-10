pub mod iterative;

pub trait Solution {
    fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[2] as &[_]), 1),
            ((&[1, 2, 3, 4], &[1, 4]), 1),
            ((&[1, 5], &[2]), 3),
            ((&[1, 5], &[10]), 9),
            (
                (
                    &[
                        282_475_249,
                        622_650_073,
                        984_943_658,
                        144_108_930,
                        470_211_272,
                        101_027_544,
                        457_850_878,
                        458_777_923,
                    ],
                    &[
                        823_564_440,
                        115_438_165,
                        784_484_492,
                        74_243_042,
                        114_807_987,
                        137_522_503,
                        441_282_327,
                        16_531_729,
                        823_378_840,
                        143_542_612,
                    ],
                ),
                161_834_419,
            ),
        ];

        for ((houses, heaters), expected) in test_cases.iter().copied() {
            assert_eq!(S::find_radius(houses.to_vec(), heaters.to_vec()), expected);
        }
    }
}

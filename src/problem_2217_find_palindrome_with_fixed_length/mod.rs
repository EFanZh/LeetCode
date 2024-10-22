pub mod mathematical;

pub trait Solution {
    fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 3, 4, 5, 90] as &[_], 3),
                &[101_i64, 111, 121, 131, 141, 999] as &[_],
            ),
            ((&[2, 4, 6], 4), &[1111, 1331, 1551]),
            (
                (
                    &[
                        392_015_495,
                        5,
                        4,
                        1,
                        425_320_571,
                        565_971_690,
                        3,
                        7,
                        6,
                        3,
                        506_222_280,
                        468_075_092,
                        5,
                    ],
                    2,
                ),
                &[-1, 55, 44, 11, -1, -1, 33, 77, 66, 33, -1, -1, 55],
            ),
            (
                (
                    &[
                        2,
                        201_429_812,
                        8,
                        520_498_110,
                        492_711_727,
                        339_882_032,
                        462_074_369,
                        9,
                        7,
                        6,
                    ],
                    1,
                ),
                &[2, -1, 8, -1, -1, -1, -1, 9, 7, 6],
            ),
        ];

        for ((queries, int_length), expected) in test_cases {
            assert_eq!(S::kth_palindrome(queries.to_vec(), int_length), expected);
        }
    }
}

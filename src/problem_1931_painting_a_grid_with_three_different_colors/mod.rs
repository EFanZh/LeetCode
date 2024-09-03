pub mod matrix_multiplication;

pub trait Solution {
    fn color_the_grid(m: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 1), 3),
            ((1, 2), 6),
            ((1, 3), 12),
            ((1, 4), 24),
            ((1, 5), 48),
            ((1, 6), 96),
            ((1, 7), 192),
            ((1, 8), 384),
            ((1, 9), 768),
            ((2, 1), 6),
            ((2, 2), 18),
            ((2, 3), 54),
            ((2, 4), 162),
            ((2, 5), 486),
            ((2, 6), 1_458),
            ((2, 7), 4_374),
            ((2, 8), 13_122),
            ((2, 9), 39_366),
            ((3, 1), 12),
            ((3, 2), 54),
            ((3, 3), 246),
            ((3, 4), 1_122),
            ((3, 5), 5_118),
            ((3, 6), 23_346),
            ((3, 7), 106_494),
            ((3, 8), 485_778),
            ((3, 9), 2_215_902),
            ((4, 1), 24),
            ((4, 2), 162),
            ((4, 3), 1_122),
            ((4, 4), 7_812),
            ((4, 5), 54_450),
            ((4, 6), 379_602),
            ((4, 7), 2_646_540),
            ((4, 8), 18_451_530),
            ((4, 9), 128_643_282),
            ((5, 1), 48),
            ((5, 2), 486),
            ((5, 3), 5_118),
            ((5, 4), 54_450),
            ((5, 5), 580_986),
            ((5, 6), 6_204_438),
            ((5, 7), 66_274_542),
            ((5, 8), 707_982_258),
            ((5, 9), 563_227_417),
        ];

        for ((m, n), expected) in test_cases {
            assert_eq!(S::color_the_grid(m, n), expected);
        }
    }
}

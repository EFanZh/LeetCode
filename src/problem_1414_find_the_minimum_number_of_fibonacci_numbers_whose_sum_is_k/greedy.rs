pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        const FIBONACCI_NUMBERS: [u32; 43] = [
            1,
            2,
            3,
            5,
            8,
            13,
            21,
            34,
            55,
            89,
            144,
            233,
            377,
            610,
            987,
            1_597,
            2_584,
            4_181,
            6_765,
            10_946,
            17_711,
            28_657,
            46_368,
            75_025,
            121_393,
            196_418,
            317_811,
            514_229,
            832_040,
            1_346_269,
            2_178_309,
            3_524_578,
            5_702_887,
            9_227_465,
            14_930_352,
            24_157_817,
            39_088_169,
            63_245_986,
            102_334_155,
            165_580_141,
            267_914_296,
            433_494_437,
            701_408_733,
        ];

        let mut k = k as u32;
        let mut end = FIBONACCI_NUMBERS.len();
        let mut result = 0;

        loop {
            let i = FIBONACCI_NUMBERS[..end].partition_point(|&x| x <= k).wrapping_sub(1);

            if let Some(&value) = FIBONACCI_NUMBERS.get(i) {
                k -= value;
                result += 1;
                end = i;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min_fibonacci_numbers(k: i32) -> i32 {
        Self::find_min_fibonacci_numbers(k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

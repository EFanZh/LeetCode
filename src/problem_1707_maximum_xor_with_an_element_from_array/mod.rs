pub mod trie;

pub trait Solution {
    fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[0, 1, 2, 3, 4] as &[_], &[[3, 1], [1, 3], [5, 6]] as &[_]),
                &[3, 3, 7] as &[_],
            ),
            ((&[5, 2, 4, 6, 6, 3], &[[12, 4], [8, 1], [6, 3]]), &[15, -1, 5]),
            (
                (
                    &[536_870_912, 0, 534_710_168, 330_218_644, 142_254_206],
                    &[
                        [558_240_772, 1_000_000_000],
                        [307_628_050, 1_000_000_000],
                        [3_319_300, 1_000_000_000],
                        [2_751_604, 683_297_522],
                        [214_004, 404_207_941],
                    ],
                ),
                &[1_050_219_420, 844_498_962, 540_190_212, 539_622_516, 330_170_208],
            ),
        ];

        for ((nums, queries), expected) in test_cases {
            assert_eq!(
                S::maximize_xor(nums.to_vec(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

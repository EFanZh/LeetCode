pub mod iterative;

pub trait Solution {
    fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ([8, 9, 4, 0, 2, 1, 3, 5, 7, 6], &[991, 338, 38] as &[_]),
                &[338, 38, 991] as &[_],
            ),
            (([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], &[789, 456, 123]), &[123, 456, 789]),
            (
                ([9, 8, 7, 6, 5, 4, 3, 2, 1, 0], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
                &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            ),
            (
                (
                    [5, 6, 8, 7, 4, 0, 3, 1, 9, 2],
                    &[7686, 97_012_948, 84_234_023, 2_212_638, 99],
                ),
                &[99, 7686, 2_212_638, 97_012_948, 84_234_023],
            ),
        ];

        for ((mapping, nums), expected) in test_cases {
            assert_eq!(S::sort_jumbled(mapping.to_vec(), nums.to_vec()), expected,);
        }
    }
}

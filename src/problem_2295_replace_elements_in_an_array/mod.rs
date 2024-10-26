pub mod hash_map;

pub trait Solution {
    fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 4, 6] as &[_], &[[1, 3], [4, 7], [6, 1]] as &[_]),
                &[3, 2, 7, 1] as &[_],
            ),
            ((&[1, 2], &[[1, 3], [2, 1], [3, 2]]), &[2, 1]),
        ];

        for ((nums, operations), expected) in test_cases {
            assert_eq!(
                S::array_change(nums.to_vec(), operations.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

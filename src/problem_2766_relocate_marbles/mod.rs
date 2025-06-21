pub mod hash_set;

pub trait Solution {
    fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 6, 7, 8] as &[_], &[1, 7, 2] as &[_], &[2, 9, 5] as &[_]),
                &[5, 6, 8, 9] as &[_],
            ),
            ((&[1, 1, 3, 3], &[1, 3], &[2, 2]), &[2]),
        ];

        for ((nums, move_from, move_to), expected) in test_cases {
            assert_eq!(
                S::relocate_marbles(nums.to_vec(), move_from.to_vec(), move_to.to_vec()),
                expected,
            );
        }
    }
}

pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2, 3] as &[_], &[4, 5, 6], &[7, 8, 9]] as &[&[_]],
                &[1, 4, 2, 7, 5, 3, 8, 6, 9] as &[_],
            ),
            (
                &[&[1, 2, 3, 4, 5], &[6, 7], &[8], &[9, 10, 11], &[12, 13, 14, 15, 16]],
                &[1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                S::find_diagonal_order(nums.iter().copied().map(<[_]>::to_vec).collect()),
                expected,
            );
        }
    }
}

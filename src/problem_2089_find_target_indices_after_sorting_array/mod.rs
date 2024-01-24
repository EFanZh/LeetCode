pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 5, 2, 3] as &[_], 2), &[1, 2] as &[_]),
            ((&[1, 2, 5, 2, 3], 3), &[3]),
            ((&[1, 2, 5, 2, 3], 5), &[4]),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::target_indices(nums.to_vec(), target), expected);
        }
    }
}

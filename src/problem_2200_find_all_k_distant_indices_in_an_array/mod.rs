pub mod iterative;

pub trait Solution {
    fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 4, 9, 1, 3, 9, 5] as &[_], 9, 1), &[1, 2, 3, 4, 5, 6] as &[_]),
            ((&[2, 2, 2, 2, 2], 2, 2), &[0, 1, 2, 3, 4]),
        ];

        for ((nums, key, k), expected) in test_cases {
            assert_eq!(S::find_k_distant_indices(nums.to_vec(), key, k), expected);
        }
    }
}

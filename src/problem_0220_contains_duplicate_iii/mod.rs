pub mod sliding_window_buckets;
pub mod sliding_window_ordered_set;

pub trait Solution {
    fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 1] as &[_], 3, 0), true),
            ((&[1, 0, 1, 1], 1, 2), true),
            ((&[1, 5, 9, 1, 5, 9], 2, 3), false),
            ((&[8, 7, 15, 1, 6, 1, 9, 15], 1, 3), true),
            ((&[-2_147_483_648, 2_147_483_647], 1, 1), false),
            ((&[2_147_483_640, 2_147_483_641], 1, 100), true),
        ];

        for ((nums, k, t), expected) in test_cases.iter().copied() {
            assert_eq!(S::contains_nearby_almost_duplicate(nums.to_vec(), k, t), expected);
        }
    }
}

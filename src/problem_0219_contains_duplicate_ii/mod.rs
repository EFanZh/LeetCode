pub mod index_map;
pub mod sliding_window_hash_set;

pub trait Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 1] as &[_], 3), true),
            ((&[1, 0, 1, 1], 1), true),
            ((&[1, 2, 3, 1, 2, 3], 2), false),
        ];

        for ((nums, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::contains_nearby_duplicate(nums.to_vec(), k), expected);
        }
    }
}

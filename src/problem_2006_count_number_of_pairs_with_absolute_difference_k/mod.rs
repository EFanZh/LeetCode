pub mod hash_map;
pub mod hash_map_2;

pub trait Solution {
    fn count_k_difference(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 2, 1] as &[_], 1), 4),
            ((&[1, 3], 3), 0),
            ((&[3, 2, 1, 5, 4], 2), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_k_difference(nums.to_vec(), k), expected);
        }
    }
}

pub mod gcd_and_hash_map;
pub mod gcd_and_hash_map_2;

pub trait Solution {
    fn count_pairs(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5] as &[_], 2), 7),
            ((&[1, 2, 3, 4], 5), 0),
            ((&[8, 10, 2, 5, 9, 6, 3, 8, 2], 6), 18),
            ((&[67, 64, 65, 66, 16, 51], 33), 5),
            ((&[5, 8], 1), 1),
            ((&[9, 2, 6, 9, 1, 2], 9), 9),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_pairs(nums.to_vec(), k), expected);
        }
    }
}

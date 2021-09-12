pub mod hash_set;

pub trait Solution {
    fn find_pairs(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 4, 1, 5] as &[_], 2), 2),
            ((&[1, 2, 3, 4, 5], 1), 4),
            ((&[1, 3, 1, 5, 4], 0), 1),
            ((&[1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3), 2),
            ((&[-1, -2, -3], 1), 2),
            ((&[1, 1, 1, 1, 1], 0), 1),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::find_pairs(nums.to_vec(), k), expected);
        }
    }
}

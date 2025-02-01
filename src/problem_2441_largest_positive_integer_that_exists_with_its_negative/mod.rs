pub mod hash_set;

pub trait Solution {
    fn find_max_k(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-1, 2, -3, 3] as &[_], 3),
            (&[-1, 10, 6, 7, -7, 1], 7),
            (&[-10, 8, 6, 7, -2, -3], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_max_k(nums.to_vec()), expected);
        }
    }
}

pub mod hash_set;

pub trait Solution {
    fn find_subarrays(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 4] as &[_], true),
            (&[1, 2, 3, 4, 5], false),
            (&[0, 0, 0], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_subarrays(nums.to_vec()), expected);
        }
    }
}

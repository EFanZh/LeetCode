pub mod hash_map;

pub trait Solution {
    fn first_unique_freq(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[20, 10, 30, 30] as &[_], 30),
            (&[20, 20, 10, 30, 30, 30], 20),
            (&[10, 10, 20, 20], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::first_unique_freq(nums.to_vec()), expected);
        }
    }
}

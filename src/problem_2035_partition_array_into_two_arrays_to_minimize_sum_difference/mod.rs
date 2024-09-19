pub mod meet_in_the_middle;

pub trait Solution {
    fn minimum_difference(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 9, 7, 3] as &[_], 2),
            (&[-36, 36], 72),
            (&[2, -1, 0, 4, -2, -9], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_difference(nums.to_vec()), expected);
        }
    }
}

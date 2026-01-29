pub mod sorting;

pub trait Solution {
    fn max_good_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3] as &[_], 30), (&[2, 8, 16], 1296)];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_good_number(nums.to_vec()), expected);
        }
    }
}

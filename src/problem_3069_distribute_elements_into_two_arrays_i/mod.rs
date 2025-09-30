pub mod brute_force;

pub trait Solution {
    fn result_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 3] as &[_], &[2, 3, 1] as &[_]), (&[5, 4, 3, 8], &[5, 3, 4, 8])];

        for (nums, expected) in test_cases {
            assert_eq!(S::result_array(nums.to_vec()), expected);
        }
    }
}

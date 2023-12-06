pub mod iterative;

pub trait Solution {
    fn find_middle_index(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, -1, 8, 4] as &[_], 3), (&[1, -1, 4], 2), (&[2, 5], -1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_middle_index(nums.to_vec()), expected);
        }
    }
}

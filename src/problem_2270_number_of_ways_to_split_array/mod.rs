pub mod iterative;

pub trait Solution {
    fn ways_to_split_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 4, -8, 7] as &[_], 2), (&[2, 3, 1, 0], 2), (&[1, 1, 1, 2, 3], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::ways_to_split_array(nums.to_vec()), expected);
        }
    }
}

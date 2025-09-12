pub mod iterative;

pub trait Solution {
    fn can_sort_array(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[8, 4, 2, 30, 15] as &[_], true),
            (&[1, 2, 3, 4, 5], true),
            (&[3, 16, 8, 4, 2], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::can_sort_array(nums.to_vec()), expected);
        }
    }
}

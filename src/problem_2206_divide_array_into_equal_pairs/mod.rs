pub mod iterative;

pub trait Solution {
    fn divide_array(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 2, 3, 2, 2, 2] as &[_], true), (&[1, 2, 3, 4], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::divide_array(nums.to_vec()), expected);
        }
    }
}

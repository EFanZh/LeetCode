pub mod dynamic_programming;

pub trait Solution {
    fn single_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 2, 3, 2] as &[_], 3), (&[0, 1, 0, 1, 0, 1, 99], 99)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::single_number(nums.to_vec()), expected);
        }
    }
}

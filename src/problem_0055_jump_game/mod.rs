pub mod greedy;

pub trait Solution {
    fn can_jump(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1, 1, 4] as &[_], true), (&[3, 2, 1, 0, 4], false)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::can_jump(nums.to_vec()), expected);
        }
    }
}

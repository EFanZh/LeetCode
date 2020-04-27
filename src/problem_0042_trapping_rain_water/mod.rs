pub mod maximum_on_both_sides;
pub mod stack;

pub trait Solution {
    fn trap(height: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1] as &[_], 6), (&[], 0)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::trap(nums.to_vec()), expected);
        }
    }
}

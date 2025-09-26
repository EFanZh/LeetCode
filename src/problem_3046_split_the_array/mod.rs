pub mod iterative;

pub trait Solution {
    fn is_possible_to_split(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 2, 2, 3, 4] as &[_], true), (&[1, 1, 1, 1], false)];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_possible_to_split(nums.to_vec()), expected);
        }
    }
}

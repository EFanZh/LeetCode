pub mod greedy;

pub trait Solution {
    fn maximize_greatness(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5, 2, 1, 3, 1] as &[_], 4), (&[1, 2, 3, 4], 3)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximize_greatness(nums.to_vec()), expected);
        }
    }
}

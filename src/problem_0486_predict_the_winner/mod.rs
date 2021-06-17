pub mod dynamic_programming;

pub trait Solution {
    fn predict_the_winner(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 5, 2] as &[_], false), (&[1, 5, 233, 7], true)];

        for (nums, expected) in test_cases {
            assert_eq!(S::predict_the_winner(nums.to_vec()), expected);
        }
    }
}

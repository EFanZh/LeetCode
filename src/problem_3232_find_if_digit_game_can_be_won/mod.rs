pub mod iterative;

pub trait Solution {
    fn can_alice_win(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 10] as &[_], false),
            (&[1, 2, 3, 4, 5, 14], true),
            (&[5, 5, 5, 25], true),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::can_alice_win(nums.to_vec()), expected);
        }
    }
}

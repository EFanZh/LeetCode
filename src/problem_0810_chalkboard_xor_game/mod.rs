pub mod iterative;

pub trait Solution {
    fn xor_game(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 2] as &[_], false),
            (&[0, 1], true),
            (&[1, 2, 3], true),
            (
                &[
                    1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1,
                    1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0,
                ],
                true,
            ),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::xor_game(nums.to_vec()), expected);
        }
    }
}

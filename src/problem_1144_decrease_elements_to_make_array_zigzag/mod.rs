pub mod iterative;

pub trait Solution {
    fn moves_to_make_zigzag(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], 2),
            (&[9, 6, 1, 6, 2], 4),
            (&[1, 2, 1, 2], 0),
            (&[2, 1, 2, 1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::moves_to_make_zigzag(nums.to_vec()), expected);
        }
    }
}

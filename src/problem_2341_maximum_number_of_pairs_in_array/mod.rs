pub mod iterative;

pub trait Solution {
    fn number_of_pairs(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 2, 1, 3, 2, 2] as &[_], [3, 1]),
            (&[1, 1], [1, 0]),
            (&[0], [0, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::number_of_pairs(nums.to_vec()), expected);
        }
    }
}

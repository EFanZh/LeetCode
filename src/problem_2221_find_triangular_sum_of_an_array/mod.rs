pub mod mod_inverse;

pub trait Solution {
    fn triangular_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 5] as &[_], 8),
            (&[5], 5),
            (&[2, 6, 6, 5, 5, 3, 3, 8, 6, 4, 3, 3, 5, 1, 0, 1, 3, 6, 9], 0),
            (&[2, 6, 6, 2, 5, 7], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::triangular_sum(nums.to_vec()), expected);
        }
    }
}

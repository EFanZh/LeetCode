pub mod greedy;

pub trait Solution {
    fn maximum_triplet_value(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[12, 6, 1, 2, 7] as &[_], 77),
            (&[1, 10, 3, 4, 19], 133),
            (&[1, 2, 3], 0),
            (&[2, 3, 1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_triplet_value(nums.to_vec()), expected);
        }
    }
}

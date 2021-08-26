pub mod greedy;

pub trait Solution {
    fn dominant_index(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 6, 1, 0] as &[_], 1),
            (&[1, 2, 3, 4], -1),
            (&[1], 0),
            (&[0, 0, 0, 1], 3),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::dominant_index(nums.to_vec()), expected);
        }
    }
}

pub mod buckets;

pub trait Solution {
    fn max_sum(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[112, 131, 411] as &[_], -1),
            (&[2536, 1613, 3366, 162], 5902),
            (&[51, 71, 17, 24, 42], 88),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_sum(nums.to_vec()), expected);
        }
    }
}

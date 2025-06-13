pub mod greedy;

pub trait Solution {
    fn find_non_min_or_max(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 2, 1, 4] as &[_], &[2, 3] as &[_]),
            (&[1, 2], &[-1]),
            (&[2, 1, 3], &[2]),
        ];

        for (nums, expected) in test_cases {
            assert!(expected.contains(&S::find_non_min_or_max(nums.to_vec())));
        }
    }
}

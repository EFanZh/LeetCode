pub mod buckets;

pub trait Solution {
    fn destroy_targets(nums: Vec<i32>, space: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 7, 8, 1, 1, 5] as &[_], 2), 1),
            ((&[1, 3, 5, 2, 4, 6], 2), 1),
            ((&[6, 2, 5], 100), 2),
            ((&[4, 5, 5, 2, 1], 4), 1),
            ((&[82, 614], 4), 82),
        ];

        for ((nums, space), expected) in test_cases {
            assert_eq!(S::destroy_targets(nums.to_vec(), space), expected);
        }
    }
}

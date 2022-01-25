pub mod iterative;

pub trait Solution {
    fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1] as &[_], 0), 0),
            ((&[0, 10], 2), 6),
            ((&[1, 3, 6], 3), 0),
            ((&[9, 10, 0, 7], 1), 8),
            ((&[5, 6, 4], 5), 0),
            ((&[6, 6, 7, 6], 4), 0),
            ((&[3, 1, 10], 4), 1),
            ((&[2, 7, 2], 1), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::smallest_range_i(nums.to_vec(), k), expected);
        }
    }
}

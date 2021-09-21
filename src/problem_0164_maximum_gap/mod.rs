pub mod buckets_and_pigeonhole;
pub mod comparison_sort;
pub mod radix_sort;

pub trait Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 6, 9, 1] as &[_], 3),
            (&[10], 0),
            (&[1, 3, 100], 97),
            (&[1, 1, 1, 1], 0),
            (&[1, 11], 10),
            (&[1, 1, 1, 1, 1, 5, 5, 5, 5, 5], 4),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_gap(nums.to_vec()), expected);
        }
    }
}

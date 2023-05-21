pub mod binary_heap;
pub mod binary_search;

pub trait Solution {
    fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            // ((&[1, 2, 3, 4] as &[_], 4, 1, 5), 13),
            // ((&[1, 2, 3, 4], 4, 3, 4), 6),
            // ((&[1, 2, 3, 4], 4, 1, 10), 50),
            ((&[7, 5, 8, 5, 6, 4, 3, 3], 8, 2, 6), 23),
        ];

        for ((nums, n, left, right), expected) in test_cases {
            assert_eq!(S::range_sum(nums.to_vec(), n, left, right), expected);
        }
    }
}

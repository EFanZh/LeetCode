pub mod cheating;
pub mod cheating_unstable;
pub mod counting_sort_and_unstable_sort_hybrid;
pub mod heap_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod quick_sort_with_three_way_partition;
pub mod radix_sort;

pub trait Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[5, 2, 3, 1] as &[_], &[1, 2, 3, 5] as &[_]),
            (&[5, 1, 1, 2, 0, 0], &[0, 0, 1, 1, 2, 5]),
            (&[5, 1, 1, 2, 0, 7], &[0, 1, 1, 2, 5, 7]),
            (&[], &[]),
            (&[7], &[7]),
            (&[7, 7], &[7, 7]),
            (&[-1, 2, -8, -10], &[-10, -8, -1, 2]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sort_array(nums.to_vec()), expected);
        }
    }
}

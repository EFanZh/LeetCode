pub mod dynamic_programming;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 1, 1, 1] as &[_], 3), 5),
            ((&[1000], -1000), 1),
            ((&[1, 2, 7, 9, 981], 1_000_000_000), 0),
        ];

        for ((nums, s), expected) in test_cases {
            assert_eq!(S::find_target_sum_ways(nums.to_vec(), s), expected);
        }
    }
}

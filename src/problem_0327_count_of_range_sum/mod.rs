pub mod merge_sort;

pub trait Solution {
    fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-2, 5, -1] as &[_], -2, 2), 3),
            ((&[-2_147_483_647, 0, -2_147_483_647, 2_147_483_647], -564, 3864), 3),
            ((&[], 0, 1), 0),
            ((&[1], 0, 1), 1),
            ((&[1, 2], 1, 3), 3),
        ];

        for ((nums, lower, upper), expected) in test_cases {
            assert_eq!(S::count_range_sum(nums.to_vec(), lower, upper), expected);
        }
    }
}

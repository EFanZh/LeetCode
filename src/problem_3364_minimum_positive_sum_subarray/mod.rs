pub mod prefix_sums;

pub trait Solution {
    fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, -2, 1, 4] as &[_], 2, 3), 1),
            ((&[-2, 2, -3, 1], 2, 3), -1),
            ((&[1, 2, 3, 4], 2, 4), 3),
        ];

        for ((nums, l, r), expected) in test_cases {
            assert_eq!(S::minimum_sum_subarray(nums.to_vec(), l, r), expected);
        }
    }
}

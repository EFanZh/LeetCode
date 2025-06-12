pub mod there_is_no_collision;

pub trait Solution {
    fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-2, 0, 2] as &[_], "RLL", 3), 8),
            ((&[1, 0], "RL", 2), 5),
            ((&[2_000_000_000, -2_000_000_000], "RL", 1_000_000_000), 999_999_965),
        ];

        for ((nums, s, d), expected) in test_cases {
            assert_eq!(S::sum_distance(nums.to_vec(), s.to_string(), d), expected);
        }
    }
}

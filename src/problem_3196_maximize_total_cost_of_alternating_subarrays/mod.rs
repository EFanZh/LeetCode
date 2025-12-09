pub mod dynamic_programming;

pub trait Solution {
    fn maximum_total_cost(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, -2, 3, 4] as &[_], 10),
            (&[1, -1, 1, -1], 4),
            (&[0], 0),
            (&[-937], -937),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_total_cost(nums.to_vec()), expected);
        }
    }
}

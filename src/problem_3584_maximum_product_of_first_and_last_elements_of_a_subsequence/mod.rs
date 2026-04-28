pub mod greedy;

pub trait Solution {
    fn maximum_product(nums: Vec<i32>, m: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[-1, -9, 2, 3, -2, -3, 1] as &[_], 1), 81),
            ((&[1, 3, -5, 5, 6, -4], 3), 20),
            ((&[2, -1, 2, -6, 5, 2, -5, 7], 2), 35),
            ((&[-1], 1), 1),
        ];

        for ((nums, m), expected) in test_cases {
            assert_eq!(S::maximum_product(nums.to_vec(), m), expected);
        }
    }
}

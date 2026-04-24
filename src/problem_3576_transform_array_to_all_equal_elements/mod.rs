pub mod greedy;

pub trait Solution {
    fn can_make_equal(nums: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, -1, 1, -1, 1] as &[_], 3), true),
            ((&[-1, -1, -1, 1, 1, 1], 5), false),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::can_make_equal(nums.to_vec(), k), expected);
        }
    }
}

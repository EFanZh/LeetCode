pub mod quick_select;

pub trait Solution {
    fn max_score(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, -1, 0, 1, -3, 3, -3] as &[_], 6),
            (&[-2, -3, 0], 0),
            (&[-32_495, -144_584, -270_506, -394_309, -298_138, 922_535], 5),
            (&[1, -4, 3, 0], 3),
            (&[2, -1, 0, 1, -3, 3, -3], 6),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_score(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn maximum_score(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 3, 7, 4, 5] as &[_], 3), 15),
            ((&[5, 5, 4, 5, 4, 1, 1, 1], 0), 20),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximum_score(nums.to_vec(), k), expected);
        }
    }
}

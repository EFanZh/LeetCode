pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn sum_of_unique(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 2] as &[_], 4),
            (&[1, 1, 1, 1, 1], 0),
            (&[1, 2, 3, 4, 5], 15),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_of_unique(nums.to_vec()), expected);
        }
    }
}

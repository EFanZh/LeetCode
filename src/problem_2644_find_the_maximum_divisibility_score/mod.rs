pub mod iterative;

pub trait Solution {
    fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 9, 15, 50] as &[_], &[5, 3, 7, 2] as &[_]), 2),
            ((&[4, 7, 9, 3, 9], &[5, 2, 3]), 3),
            ((&[20, 14, 21, 10], &[10, 16, 20]), 10),
            ((&[1], &[5, 7, 5]), 5),
        ];

        for ((nums, divisors), expected) in test_cases {
            assert_eq!(S::max_div_score(nums.to_vec(), divisors.to_vec()), expected);
        }
    }
}

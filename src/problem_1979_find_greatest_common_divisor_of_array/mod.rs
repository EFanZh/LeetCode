pub mod iterative;

pub trait Solution {
    fn find_gcd(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 6, 9, 10] as &[_], 2),
            (&[7, 5, 6, 8, 3], 1),
            (&[3, 3], 3),
            (&[10, 7], 1),
            (&[1, 5], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_gcd(nums.to_vec()), expected);
        }
    }
}

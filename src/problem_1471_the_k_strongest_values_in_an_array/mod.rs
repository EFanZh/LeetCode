pub mod reuse_upper_bits;

pub trait Solution {
    fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5, 1, 3, 4, 7] as &[_], 3), &[2, 3, 5, 4, 1, 7] as &[_]),
            ((&[1, 2, 3, 4, 4, 3, 2, 1], 4), &[1, 4, 2, 3, 3, 2, 4, 1]),
            ((&[1, 1, 2, 2], 2), &[1, 2, 1, 2]),
        ];

        for ((nums, n), expected) in test_cases {
            assert_eq!(S::shuffle(nums.to_vec(), n), expected);
        }
    }
}

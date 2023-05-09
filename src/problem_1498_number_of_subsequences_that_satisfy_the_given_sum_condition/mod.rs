pub mod two_pointers;

pub trait Solution {
    fn num_subseq(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 5, 6, 7] as &[_], 9), 4),
            ((&[3, 3, 6, 8], 10), 6),
            ((&[2, 3, 3, 4, 6, 7], 12), 61),
            ((&[5, 2, 4, 1, 7, 6, 8], 16), 127),
        ];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::num_subseq(nums.to_vec(), target), expected);
        }
    }
}

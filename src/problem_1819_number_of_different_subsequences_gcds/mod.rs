pub mod mathematical;

pub trait Solution {
    fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[6, 10, 3] as &[_], 5), (&[5, 15, 40, 5, 6], 7), (&[19, 3, 9], 4)];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_different_subsequence_gc_ds(nums.to_vec()), expected);
        }
    }
}

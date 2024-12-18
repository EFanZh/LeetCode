pub mod iterative;

pub trait Solution {
    fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3, 1] as &[_], 3), 5), ((&[5, 1, 1], 10), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::count_excellent_pairs(nums.to_vec(), k), expected);
        }
    }
}

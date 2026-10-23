pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn max_sum(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 6, 5, 8] as &[_], 2), 261), ((&[4, 5, 4, 7], 3), 90)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::max_sum(nums.to_vec(), k), expected);
        }
    }
}

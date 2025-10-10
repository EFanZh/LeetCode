pub mod greedy;

pub trait Solution {
    fn maximize_sum(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3, 4, 5] as &[_], 3), 18), ((&[5, 5, 5], 2), 11)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::maximize_sum(nums.to_vec(), k), expected);
        }
    }
}

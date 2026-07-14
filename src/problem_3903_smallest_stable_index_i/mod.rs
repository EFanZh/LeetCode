pub mod iterative;

pub trait Solution {
    fn first_stable_index(nums: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 0, 1, 4] as &[_], 3), 3), ((&[3, 2, 1], 1), -1), ((&[0], 0), 0)];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::first_stable_index(nums.to_vec(), k), expected);
        }
    }
}

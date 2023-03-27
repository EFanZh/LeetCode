pub mod greedy;

pub trait Solution {
    fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 1, 1, 1, 1] as &[_], 2), 2), ((&[-1, 3, 5, 1, 4, 2, -9], 6), 2)];

        for ((nums, target), expected) in test_cases {
            assert_eq!(S::max_non_overlapping(nums.to_vec(), target), expected);
        }
    }
}

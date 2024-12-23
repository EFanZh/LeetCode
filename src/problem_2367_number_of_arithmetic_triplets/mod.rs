pub mod iterative;

pub trait Solution {
    fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[0, 1, 4, 6, 7, 10] as &[_], 3), 2), ((&[4, 5, 6, 7, 8, 9], 2), 2)];

        for ((nums, diff), expected) in test_cases {
            assert_eq!(S::arithmetic_triplets(nums.to_vec(), diff), expected);
        }
    }
}

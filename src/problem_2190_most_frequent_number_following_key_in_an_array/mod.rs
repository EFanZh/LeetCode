pub mod iterative;

pub trait Solution {
    fn most_frequent(nums: Vec<i32>, key: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 100, 200, 1, 100] as &[_], 1), 100), ((&[2, 2, 2, 2, 3], 2), 2)];

        for ((nums, key), expected) in test_cases {
            assert_eq!(S::most_frequent(nums.to_vec(), key), expected);
        }
    }
}

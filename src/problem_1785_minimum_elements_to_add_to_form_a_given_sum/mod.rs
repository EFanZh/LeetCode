pub mod mathematical;

pub trait Solution {
    fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, -1, 1] as &[_], 3, -4), 2), ((&[1, -10, 9, 1], 100, 0), 1)];

        for ((nums, limit, goal), expected) in test_cases {
            assert_eq!(S::min_elements(nums.to_vec(), limit, goal), expected);
        }
    }
}

pub mod sort_then_binary_search;

pub trait Solution {
    fn triangle_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 2, 3, 4] as &[_], 3), (&[4, 2, 3, 4], 4), (&[0, 0, 0], 0)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::triangle_number(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn last_visited_integers(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, -1, -1, -1] as &[_], &[2, 1, -1] as &[_]),
            (&[1, -1, 2, -1, -1], &[1, 2, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::last_visited_integers(nums.to_vec()), expected);
        }
    }
}

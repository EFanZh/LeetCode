pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn ways_to_make_fair(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 1, 6, 4] as &[_], 1), (&[1, 1, 1], 3), (&[1, 2, 3], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::ways_to_make_fair(nums.to_vec()), expected);
        }
    }
}

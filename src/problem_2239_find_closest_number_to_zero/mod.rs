pub mod iterative;

pub trait Solution {
    fn find_closest_number(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[-4, -2, 1, 4, 8] as &[_], 1), (&[2, -1, 1], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_closest_number(nums.to_vec()), expected);
        }
    }
}

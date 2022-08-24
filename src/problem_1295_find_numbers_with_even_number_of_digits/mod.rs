pub mod iterative;

pub trait Solution {
    fn find_numbers(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[12, 345, 2, 6, 7896] as &[_], 2), (&[555, 901, 482, 1771], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_numbers(nums.to_vec()), expected);
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn count_elements(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[11, 7, 2, 15] as &[_], 2),
            (&[-3, 3, 3, 90], 2),
            (&[0], 0),
            (&[-71, -71, 93, -71, 40], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::count_elements(nums.to_vec()), expected);
        }
    }
}

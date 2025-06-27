pub mod pigeonhole_principle;

pub trait Solution {
    fn is_good(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 3] as &[_], false),
            (&[1, 3, 3, 2], true),
            (&[1, 1], true),
            (&[3, 4, 4, 1, 2, 1], false),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::is_good(nums.to_vec()), expected);
        }
    }
}

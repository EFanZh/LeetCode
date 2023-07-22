pub mod sliding_window;

pub trait Solution {
    fn ways_to_split(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 1] as &[_], 1),
            (&[1, 2, 2, 2, 5, 0], 3),
            (&[3, 2, 1], 0),
            (&[0, 3, 3], 1),
            (&[0, 0, 0, 0, 0, 0, 0, 0], 21),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::ways_to_split(nums.to_vec()), expected);
        }
    }
}

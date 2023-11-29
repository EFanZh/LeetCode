pub mod stack;

pub trait Solution {
    fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[10, 6, 8, 5, 11, 9] as &[_], &[3, 1, 2, 1, 1, 0] as &[_]),
            (&[5, 1, 2, 3, 10], &[4, 1, 1, 1, 0]),
        ];

        for (heights, expected) in test_cases {
            assert_eq!(S::can_see_persons_count(heights.to_vec()), expected);
        }
    }
}

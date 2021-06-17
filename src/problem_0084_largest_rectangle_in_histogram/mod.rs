pub mod stack;
pub mod stack_2;

pub trait Solution {
    fn largest_rectangle_area(heights: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 1, 5, 6, 2, 3] as &[_], 10),
            (&[], 0),
            (&[4], 4),
            (&[0], 0),
            (&[0, 0], 0),
            (&[0, 2], 2),
            (&[2, 0], 2),
            (&[2, 3], 4),
            (&[3, 2], 4),
            (&[3, 2], 4),
        ];

        for (heights, expected) in test_cases {
            assert_eq!(S::largest_rectangle_area(heights.to_vec()), expected);
        }
    }
}

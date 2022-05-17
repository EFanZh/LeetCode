pub mod counting_sort;

pub trait Solution {
    fn height_checker(heights: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 4, 2, 1, 3] as &[_], 3),
            (&[5, 1, 2, 3, 4], 5),
            (&[1, 2, 3, 4, 5], 0),
        ];

        for (heights, expected) in test_cases {
            assert_eq!(S::height_checker(heights.to_vec()), expected);
        }
    }
}

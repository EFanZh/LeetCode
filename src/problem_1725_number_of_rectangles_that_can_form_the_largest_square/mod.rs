pub mod iterative;

pub trait Solution {
    fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[5, 8], [3, 9], [5, 12], [16, 5]] as &[_], 3),
            (&[[2, 3], [3, 7], [4, 3], [3, 7]], 3),
        ];

        for (rectangles, expected) in test_cases {
            assert_eq!(
                S::count_good_rectangles(rectangles.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

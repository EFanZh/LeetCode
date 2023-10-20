pub mod iterative;

pub trait Solution {
    fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2], [3, 4], [5, 6]] as &[_], 2, 5), true),
            ((&[[1, 10], [10, 20]], 21, 21), false),
        ];

        for ((ranges, left, right), expected) in test_cases {
            assert_eq!(
                S::is_covered(ranges.iter().copied().map(Vec::from).collect(), left, right),
                expected,
            );
        }
    }
}

pub mod iterative;

pub trait Solution {
    fn count_ways(ranges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[6, 10], [5, 15]] as &[_], 2),
            (&[[1, 3], [10, 20], [2, 5], [4, 8]], 4),
        ];

        for (ranges, expected) in test_cases {
            assert_eq!(S::count_ways(ranges.iter().map(Vec::from).collect()), expected);
        }
    }
}

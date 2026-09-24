pub mod greedy;

pub trait Solution {
    fn max_distance(arrays: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 2, 3] as &[_], &[4, 5], &[1, 2, 3]] as &[&[_]], 4),
            (&[&[1], &[1]], 0),
            (&[&[-1, 1], &[-3, 1, 4], &[-2, -1, 0, 2]], 6),
        ];

        for (arrays, expected) in test_cases {
            assert_eq!(
                S::max_distance(arrays.iter().copied().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

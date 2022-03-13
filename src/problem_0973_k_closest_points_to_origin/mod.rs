pub mod quick_select;

pub trait Solution {
    fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 3], [-2, 2]] as &[_], 1), &[[-2, 2]] as &[_]),
            ((&[[3, 3], [5, -1], [-2, 4]], 2), &[[-2, 4], [3, 3]]),
            ((&[[0, 1], [1, 0]], 2), &[[0, 1], [1, 0]]),
        ];

        for ((points, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::k_closest(points.iter().copied().map(Vec::from).collect(), k)),
                expected
            );
        }
    }
}

pub mod greedy;

pub trait Solution {
    fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [3, 4]] as &[_], 2),
            (&[[1, 2], [7, 8], [4, 5]], 3),
            (
                &[[-10, -8], [8, 9], [-5, 0], [6, 10], [-6, -4], [1, 7], [9, 10], [-4, 7]],
                4,
            ),
        ];

        for (pairs, expected) in test_cases {
            assert_eq!(
                S::find_longest_chain(pairs.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

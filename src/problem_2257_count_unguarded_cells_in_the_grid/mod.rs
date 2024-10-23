pub mod iterative;

pub trait Solution {
    fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    4,
                    6,
                    &[[0, 0], [1, 1], [2, 3]] as &[_],
                    &[[0, 1], [2, 2], [1, 4]] as &[_],
                ),
                7,
            ),
            ((3, 3, &[[1, 1]], &[[0, 1], [1, 0], [2, 1], [1, 2]]), 4),
        ];

        for ((m, n, guards, walls), expected) in test_cases {
            assert_eq!(
                S::count_unguarded(
                    m,
                    n,
                    guards.iter().map(Vec::from).collect(),
                    walls.iter().map(Vec::from).collect(),
                ),
                expected,
            );
        }
    }
}

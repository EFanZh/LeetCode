pub mod recursive;

pub trait Solution {
    fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[0, 1], [1, 2], [1, 3], [3, 4]] as &[_], 3, &[-2, 4, 2, -4, 6] as &[_]),
                6,
            ),
            ((&[[0, 1]], 1, &[-7280, 2350]), -7280),
        ];

        for ((edges, bob, amount), expected) in test_cases {
            assert_eq!(
                S::most_profitable_path(edges.iter().map(Vec::from).collect(), bob, amount.to_vec()),
                expected,
            );
        }
    }
}

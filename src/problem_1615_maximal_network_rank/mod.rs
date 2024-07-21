pub mod brute_force;

pub trait Solution {
    fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[0, 1], [0, 3], [1, 2], [1, 3]] as &[_]), 4),
            ((5, &[[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]]), 5),
            ((8, &[[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]]), 5),
            ((2, &[[1, 0]]), 1),
        ];

        for ((n, roads), expected) in test_cases {
            assert_eq!(
                S::maximal_network_rank(n, roads.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

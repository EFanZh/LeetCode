pub mod bfs;

pub trait Solution {
    fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]] as &[_], 3, 5), 13),
            ((2, &[[1, 2]], 3, 2), 11),
        ];

        for ((n, edges, time, change), expected) in test_cases {
            assert_eq!(
                S::second_minimum(n, edges.iter().map(Vec::from).collect(), time, change),
                expected,
            );
        }
    }
}

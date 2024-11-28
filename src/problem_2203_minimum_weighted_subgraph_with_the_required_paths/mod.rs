pub mod dijkstra;

pub trait Solution {
    fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    6,
                    &[
                        [0, 2, 2],
                        [0, 5, 6],
                        [1, 0, 3],
                        [1, 4, 5],
                        [2, 1, 1],
                        [2, 3, 3],
                        [2, 3, 4],
                        [3, 4, 2],
                        [4, 5, 1],
                    ] as &[_],
                    0,
                    1,
                    5,
                ),
                9,
            ),
            ((3, &[[0, 1, 1], [2, 1, 1]], 0, 1, 2), -1),
        ];

        for ((n, edges, src1, src2, dest), expected) in test_cases {
            assert_eq!(
                S::minimum_weight(n, edges.iter().map(Vec::from).collect(), src1, src2, dest),
                expected,
            );
        }
    }
}

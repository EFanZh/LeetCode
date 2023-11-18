pub mod brute_force;

pub trait Solution {
    fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[1, 2], [2, 3], [2, 4]] as &[_]), &[3, 4, 0] as &[_]),
            ((2, &[[1, 2]]), &[1]),
            ((3, &[[1, 2], [2, 3]]), &[2, 1]),
            (
                (8, &[[1, 5], [2, 3], [2, 5], [2, 8], [4, 7], [6, 7], [6, 8]]),
                &[7, 8, 8, 6, 5, 2, 0],
            ),
            (
                (9, &[[1, 3], [1, 5], [1, 6], [2, 4], [3, 4], [5, 7], [5, 8], [6, 9]]),
                &[8, 11, 16, 20, 11, 0, 0, 0],
            ),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                S::count_subgraphs_for_each_diameter(n, edges.iter().map(Vec::from).collect(),),
                expected,
            );
        }
    }
}

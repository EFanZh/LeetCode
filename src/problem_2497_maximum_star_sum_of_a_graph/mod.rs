pub mod greedy;

pub trait Solution {
    fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[1, 2, 3, 4, 10, -10, -20] as &[_],
                    &[[0, 1], [1, 2], [1, 3], [3, 4], [3, 5], [3, 6]] as &[_],
                    2,
                ),
                16,
            ),
            ((&[-5], &[], 0), -5),
        ];

        for ((vals, edges, k), expected) in test_cases {
            assert_eq!(
                S::max_star_sum(vals.to_vec(), edges.iter().map(Vec::from).collect(), k),
                expected,
            );
        }
    }
}

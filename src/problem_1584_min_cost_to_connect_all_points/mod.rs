pub mod greedy_kruskal;
pub mod greedy_prim;

pub trait Solution {
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]] as &[_], 20),
            (&[[3, 12], [-2, 5], [-4, 1]], 18),
        ];

        for (points, expected) in test_cases {
            assert_eq!(
                S::min_cost_connect_points(points.iter().map(Vec::from).collect()),
                expected
            );
        }
    }
}

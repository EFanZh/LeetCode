pub mod dijkstra;

pub trait Solution {
    fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]] as &[_]), 5),
            ((4, &[[0, 2, 1], [2, 1, 1], [1, 3, 1], [2, 3, 3]]), 3),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(S::min_cost(n, edges.iter().map(Vec::from).collect()), expected,);
        }
    }
}

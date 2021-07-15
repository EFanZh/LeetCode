pub mod union_find;

pub trait Solution {
    fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [1, 3], [2, 3]] as &[_], [2, 3]),
            (&[[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]], [1, 4]),
        ];

        for (edges, expected) in test_cases {
            assert_eq!(
                S::find_redundant_connection(edges.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

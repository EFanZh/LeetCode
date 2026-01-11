pub mod binary_heap;

pub trait Solution {
    fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 2], [3, 4], [2, 3], [-3, 0]] as &[_], 2), &[-1, 7, 5, 3] as &[_]),
            ((&[[5, 5], [4, 4], [3, 3]], 1), &[10, 8, 6]),
            ((&[[-7, -1]], 3), &[-1]),
            ((&[[-6, 4], [7, 8], [-2, -1], [1, -9], [-9, 4]], 1), &[10, 10, 3, 3, 3]),
        ];

        for ((queries, k), expected) in test_cases {
            assert_eq!(S::results_array(queries.iter().map(Vec::from).collect(), k), expected,);
        }
    }
}

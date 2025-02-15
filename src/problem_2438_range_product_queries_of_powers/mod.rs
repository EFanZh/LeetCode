pub mod prefix_sums;

pub trait Solution {
    fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((15, &[[0, 1], [2, 2], [0, 3]] as &[_]), &[2, 4, 64] as &[_]),
            ((2, &[[0, 0]]), &[2]),
        ];

        for ((n, queries), expected) in test_cases {
            assert_eq!(S::product_queries(n, queries.iter().map(Vec::from).collect()), expected);
        }
    }
}

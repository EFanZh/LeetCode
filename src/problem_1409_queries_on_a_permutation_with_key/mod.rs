pub mod fenwick_tree;

pub trait Solution {
    fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 2, 1] as &[_], 5), &[2, 1, 2, 1] as &[_]),
            ((&[4, 1, 2, 2], 4), &[3, 1, 2, 0]),
            ((&[7, 5, 5, 8, 3], 8), &[6, 5, 0, 7, 5]),
        ];

        for ((queries, m), expected) in test_cases {
            assert_eq!(S::process_queries(queries.to_vec(), m), expected);
        }
    }
}

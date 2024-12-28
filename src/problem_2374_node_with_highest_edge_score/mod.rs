pub mod iterative;

pub trait Solution {
    fn edge_score(edges: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 0, 0, 0, 0, 7, 7, 5] as &[_], 7), (&[2, 0, 0, 2], 0)];

        for (edges, expected) in test_cases {
            assert_eq!(S::edge_score(edges.to_vec()), expected);
        }
    }
}

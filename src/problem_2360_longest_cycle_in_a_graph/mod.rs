pub mod iterative;

pub trait Solution {
    fn longest_cycle(edges: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 3, 4, 2, 3] as &[_], 3),
            (&[2, -1, 3, 1], -1),
            (&[3, 3, 4, 2, 3], 3),
            (&[4, 3, 3, 4, 7, 2, 3, 3], 3),
        ];

        for (edges, expected) in test_cases {
            assert_eq!(S::longest_cycle(edges.to_vec()), expected);
        }
    }
}

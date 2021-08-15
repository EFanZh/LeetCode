pub mod btree_map;
pub mod segment_tree;

pub trait Solution {
    fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [6, 1]] as &[_], &[2, 5, 5] as &[_]),
            (&[[100, 100], [200, 100]], &[100, 100]),
            (&[[6, 4], [2, 7], [6, 9]], &[4, 11, 20]),
            (&[[6, 1], [9, 2], [2, 4]], &[1, 2, 4]),
            (&[[1, 5], [2, 2], [7, 5]], &[5, 7, 7]),
            (&[[4, 9], [8, 8], [6, 8], [8, 2], [1, 2]], &[9, 17, 25, 27, 27]),
        ];

        for (positions, expected) in test_cases {
            assert_eq!(
                S::falling_squares(positions.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}

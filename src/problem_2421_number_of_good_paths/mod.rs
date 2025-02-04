pub mod union_find;

pub trait Solution {
    fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2, 1, 3] as &[_], &[[0, 1], [0, 2], [2, 3], [2, 4]] as &[_]), 6),
            ((&[1, 1, 2, 2, 3], &[[0, 1], [1, 2], [2, 3], [2, 4]]), 7),
            ((&[1], &[]), 1),
            (
                (
                    &[2, 4, 1, 2, 2, 5, 3, 4, 4],
                    &[[0, 1], [2, 1], [0, 3], [4, 1], [4, 5], [3, 6], [7, 5], [2, 8]],
                ),
                11,
            ),
        ];

        for ((vals, edges), expected) in test_cases {
            assert_eq!(
                S::number_of_good_paths(vals.to_vec(), edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}

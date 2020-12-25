pub mod bfs;
pub mod bfs_2;
pub mod binary_search;
pub mod binary_search_2;
pub mod binary_search_3;

pub trait Solution {
    fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[&[1, 5, 9] as &[_], &[10, 11, 13], &[12, 13, 15]] as &[&[_]], 8), 13)];

        for ((matrix, k), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::kth_smallest(matrix.iter().map(|row| row.to_vec()).collect(), k),
                expected
            );
        }
    }
}

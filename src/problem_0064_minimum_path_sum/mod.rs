pub mod dynamic_programming;

pub trait Solution {
    fn min_path_sum(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[&[1, 3, 1] as &[_], &[1, 5, 1], &[4, 2, 1]] as &[&[_]], 7)];

        for (grid, expected) in test_cases.iter().copied() {
            assert_eq!(S::min_path_sum(grid.iter().map(|row| row.to_vec()).collect()), expected);
        }
    }
}

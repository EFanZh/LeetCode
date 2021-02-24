pub mod brute_force;

pub trait Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[0, 1, 0, 0] as &[_], &[1, 1, 1, 0], &[0, 1, 0, 0], &[1, 1, 0, 0]] as &[&[_]],
                16,
            ),
            (&[&[1]], 4),
            (&[&[1, 0]], 4),
        ];

        for (grid, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::island_perimeter(grid.iter().map(|row| row.to_vec()).collect()),
                expected
            );
        }
    }
}

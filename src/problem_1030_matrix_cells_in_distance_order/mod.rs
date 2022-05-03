pub mod bfs;

pub trait Solution {
    fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 2, 0, 0), (2, 2, 0, 1), (2, 3, 1, 2), (3, 5, 2, 3)];

        for (rows, cols, r_center, c_center) in test_cases {
            let result = S::all_cells_dist_order(rows, cols, r_center, c_center)
                .into_iter()
                .map(|cell| cell.try_into().unwrap())
                .collect::<Vec<_>>();

            let expected_set = (0..rows)
                .flat_map(|y| (0..cols).map(move |x| [y, x]))
                .collect::<HashSet<_>>();

            assert_eq!(result.iter().copied().collect::<HashSet<_>>(), expected_set);

            let mut iter = result
                .into_iter()
                .map(|[y, x]| (y - r_center).abs() + (x - c_center).abs());

            let mut prev = iter.next().unwrap();

            for distance in iter {
                assert!(prev <= distance);

                prev = distance;
            }
        }
    }
}

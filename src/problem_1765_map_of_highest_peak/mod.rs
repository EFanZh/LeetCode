pub mod bfs;

pub trait Solution {
    fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1], [0, 0]] as &dyn Matrix<_>, 2),
            (&[[0, 0, 1], [1, 0, 0], [0, 0, 0]], 2),
        ];

        for (is_water, expected_max_height) in test_cases {
            let is_water = is_water.to_vec();
            let result = S::highest_peak(is_water.clone());

            assert_eq!(result.len(), is_water.len());

            let mut max_height = 0;

            for (y, (result_row, input_row)) in result.iter().zip(&is_water).enumerate() {
                assert_eq!(result_row.len(), input_row.len());

                for (x, (&result_height, &cell_is_water)) in result_row.iter().zip(input_row).enumerate() {
                    if cell_is_water == 0 {
                        assert!(result_height > 0);

                        for neighbor in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                            if let Some(&neighbor_height) = result.get(neighbor.0).and_then(|row| row.get(neighbor.1)) {
                                assert!((result_height - neighbor_height).abs() < 2);
                            }
                        }

                        max_height = max_height.max(result_height);
                    } else {
                        assert_eq!(result_height, 0);
                    }
                }
            }

            assert_eq!(max_height, expected_max_height);
        }
    }
}

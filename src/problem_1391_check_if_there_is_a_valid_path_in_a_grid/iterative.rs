pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let target = (rows - 1, columns - 1);

        for mut direction in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
            let mut current = (0, 0);

            while let Some(shape) = grid.get(current.0).and_then(|row| row.get(current.1)) {
                match shape {
                    1 => {
                        if direction.0 != 0 {
                            break;
                        }
                    }
                    2 => {
                        if direction.1 != 0 {
                            break;
                        }
                    }
                    3 => {
                        direction = if matches!(direction, (0, 1) | (usize::MAX, 0)) {
                            (direction.1, direction.0)
                        } else {
                            break;
                        }
                    }
                    4 => {
                        direction = if direction == (0, usize::MAX) {
                            (1, 0)
                        } else if direction == (usize::MAX, 0) {
                            (0, 1)
                        } else {
                            break;
                        }
                    }
                    5 => {
                        direction = if direction == (0, 1) {
                            (usize::MAX, 0)
                        } else if direction == (1, 0) {
                            (0, usize::MAX)
                        } else {
                            break;
                        }
                    }
                    _ => {
                        direction = if matches!(direction, (0, usize::MAX) | (1, 0)) {
                            (direction.1, direction.0)
                        } else {
                            break;
                        }
                    }
                }

                if current == target {
                    return true;
                }

                current.0 = current.0.wrapping_add(direction.0);
                current.1 = current.1.wrapping_add(direction.1);

                if current == (0, 0) {
                    return false;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        Self::has_valid_path(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

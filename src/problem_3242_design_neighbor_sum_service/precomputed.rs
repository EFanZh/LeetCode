// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct NeighborSum {
    precomputed: Box<[(u16, u16)]>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut precomputed = vec![(0, 0); grid.len().pow(2)].into_boxed_slice();
        let get = |y: usize, x: usize| grid.get(y).and_then(|row| row.get(x)).copied().unwrap_or(0) as u16;

        for (y, row) in grid.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                precomputed[value.cast_unsigned() as usize] = (
                    get(y.wrapping_sub(1), x) + get(y, x.wrapping_sub(1)) + get(y, x + 1) + get(y + 1, x),
                    get(y.wrapping_sub(1), x.wrapping_sub(1))
                        + get(y.wrapping_sub(1), x + 1)
                        + get(y + 1, x.wrapping_sub(1))
                        + get(y + 1, x + 1),
                );
            }
        }

        Self { precomputed }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        i32::from(self.precomputed[value.cast_unsigned() as usize].0)
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        i32::from(self.precomputed[value.cast_unsigned() as usize].1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::NeighborSum for NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self::new(grid)
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.adjacent_sum(value)
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.diagonal_sum(value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::NeighborSum>();
    }
}

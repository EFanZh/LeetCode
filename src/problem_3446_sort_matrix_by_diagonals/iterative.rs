pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let n = grid.len();
        let mut buffer = Vec::with_capacity(n);

        for i in 1..n {
            buffer.extend(grid.iter().zip(i..).map_while(|(row, i)| row.get(i).copied()));
            buffer.sort_unstable();

            grid.iter_mut()
                .zip(i..)
                .map_while(|(row, i)| row.get_mut(i))
                .zip(&buffer)
                .for_each(|(target, &value)| *target = value);

            buffer.clear();
        }

        for i in 0..n {
            buffer.extend(grid[i..].iter().enumerate().map(|(i, row)| row[i]));
            buffer.sort_unstable();

            grid[i..]
                .iter_mut()
                .enumerate()
                .zip(buffer.iter().rev())
                .for_each(|((i, row), &value)| row[i] = value);

            buffer.clear();
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::sort_matrix(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

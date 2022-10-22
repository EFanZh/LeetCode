pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut y = 0;
        let mut x = columns.wrapping_sub(1);
        let mut non_negatives = 0;

        while let Some(&value) = grid.get(y).and_then(|row| row.get(x)) {
            if value < 0 {
                x = x.wrapping_sub(1);
            } else {
                non_negatives += x + 1;
                y += 1;
            }
        }

        ((columns * rows) - non_negatives) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        Self::count_negatives(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

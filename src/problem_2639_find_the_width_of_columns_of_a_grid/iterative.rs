pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let columns = grid.first().map_or(0, Vec::len);

        (0..columns)
            .map(|i| {
                grid.iter().fold(0, |max, row| {
                    let value = row[i];

                    max.max('block: {
                        let (abs, extra) = match value.cmp(&0) {
                            Ordering::Less => (-value, 2),
                            Ordering::Equal => break 'block 1,
                            Ordering::Greater => (value, 1),
                        };

                        (abs as u32).ilog10() + extra
                    })
                }) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_column_width(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

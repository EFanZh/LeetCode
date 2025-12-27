pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut flips_1 = 0;

        for row in &grid {
            let (left, right) = row.split_at(row.len() / 2);

            left.iter()
                .zip(right.iter().rev())
                .for_each(|(&x, &y)| flips_1 += u32::from(x != y));
        }

        let mut flips_2 = 0;
        let (top, bottom) = grid.split_at(grid.len() / 2);
        let iter = top.iter().zip(bottom.iter().rev());

        for column in 0..grid.first().map_or(0, Vec::len) {
            iter.clone()
                .for_each(|(x, y)| flips_2 += u32::from(x.get(column) != y.get(column)));
        }

        flips_1.min(flips_2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_flips(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        grid.windows(3).fold(0, |max, rows| {
            let [row_0, row_1, row_2]: &[_; 3] = rows.try_into().ok().unwrap();

            row_0
                .windows(3)
                .zip(row_1.windows(3))
                .zip(row_2.windows(3))
                .fold(max, |max, ((top, middle), bottom)| {
                    let top: &[_; 3] = top.try_into().ok().unwrap();
                    let middle: &[_; 3] = middle.try_into().ok().unwrap();
                    let bottom: &[_; 3] = bottom.try_into().ok().unwrap();

                    u32::max(
                        max,
                        (top.iter().sum::<i32>() + middle[1] + bottom.iter().sum::<i32>()) as _,
                    )
                })
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_sum(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

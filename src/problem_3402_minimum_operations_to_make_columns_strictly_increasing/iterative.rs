pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for column in 0..grid.first().map_or(0, Vec::len) {
            let mut prev = i32::MIN;

            for row in &grid {
                let value = row[column];

                if value <= prev {
                    prev += 1;
                    result += prev - value;
                } else {
                    prev = value;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        Self::minimum_operations(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

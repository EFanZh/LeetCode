pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn multiply(lhs: i32, rhs: i32) -> i32 {
        ((u64::from(lhs as u32) * u64::from(rhs as u32)) % 12345) as i32
    }

    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = grid.clone();
        let mut product = 1;

        result.iter_mut().rev().for_each(|target_row| {
            target_row
                .iter_mut()
                .rev()
                .for_each(|target_cell| (*target_cell, product) = (product, Self::multiply(product, *target_cell)));
        });

        product = 1;

        result.iter_mut().zip(grid).for_each(|(target_row, source_row)| {
            target_row
                .iter_mut()
                .zip(source_row)
                .for_each(|(target_cell, source_cell)| {
                    (*target_cell, product) = (
                        Self::multiply(*target_cell, product),
                        Self::multiply(product, source_cell),
                    );
                });
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::construct_product_matrix(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

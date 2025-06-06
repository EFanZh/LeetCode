pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_count = 0;
        let mut max_count_row_index = 0;

        (0..).zip(mat).for_each(|(i, row)| {
            let count = row.into_iter().sum::<i32>();

            if count as u32 > max_count as u32 {
                max_count = count;
                max_count_row_index = i;
            }
        });

        vec![max_count_row_index, max_count]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        Self::row_and_maximum_ones(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

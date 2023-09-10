pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as u32 as usize;
        let mut result = (1..=num_rows).map(Vec::with_capacity).collect::<Vec<_>>();
        let mut iter = result.iter_mut();
        let mut top_row = iter.next().unwrap();

        top_row.push(1);

        for row in iter {
            let mut top_left = 0;

            row.extend(top_row.iter().map(|&top| {
                let result = top_left + top;

                top_left = top;

                result
            }));

            row.push(1);

            top_row = row;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        Self::generate(num_rows)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

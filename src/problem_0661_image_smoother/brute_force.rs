pub struct Solution;

impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = m.len();
        let columns = m.first().map_or(0, Vec::len);
        let mut result = Vec::with_capacity(rows);

        for y in 0..rows {
            let mut output_row = Vec::with_capacity(columns);

            for x in 0..columns {
                let mut sum = 0;
                let mut count = 0;

                for input_row in &m[y.saturating_sub(1)..(y + 2).min(rows)] {
                    for num in &input_row[x.saturating_sub(1)..(x + 2).min(columns)] {
                        sum += num;
                        count += 1;
                    }
                }

                output_row.push(sum / count);
            }

            result.push(output_row);
        }

        result
    }
}

impl super::Solution for Solution {
    fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::image_smoother(m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

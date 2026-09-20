pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n.cast_unsigned() as usize;
        let mut result = vec![vec![0; n]; n];

        for query in queries {
            let [row_1, column_1, row_2, column_2] =
                <[_; 4]>::map(query.try_into().ok().unwrap(), |x| x.cast_unsigned() as usize);

            let row_1_vec = &mut result[row_1];

            row_1_vec[column_1] += 1;

            if let Some(target) = row_1_vec.get_mut(column_2 + 1) {
                *target -= 1;
            }

            if let Some(row_2_vec) = result.get_mut(row_2 + 1) {
                row_2_vec[column_1] -= 1;

                if let Some(target) = row_2_vec.get_mut(column_2 + 1) {
                    *target += 1;
                }
            }
        }

        let mut iter = result.iter_mut();
        let mut prev_row = iter.next().unwrap();
        let mut sum = 0;

        for target in &mut *prev_row {
            sum += *target;
            *target = sum;
        }

        for row in iter {
            sum = 0;

            row.iter_mut().zip(prev_row).for_each(|(target, &mut top)| {
                sum += *target;
                *target = sum + top;
            });

            prev_row = row;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::range_add_queries(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

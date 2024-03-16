pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let [column_0, row_0, _, column_1, row_1]: [_; 5] = s.into_bytes().try_into().ok().unwrap();
        let mut result = Vec::with_capacity(usize::from(column_1 - column_0 + 1));

        for column in column_0..=column_1 {
            for row in row_0..=row_1 {
                result.push(str::from_utf8(&[column, row]).unwrap().to_string());
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cells_in_range(s: String) -> Vec<String> {
        Self::cells_in_range(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

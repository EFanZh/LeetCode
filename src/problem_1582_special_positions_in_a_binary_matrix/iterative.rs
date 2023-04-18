pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        for (y, row) in mat.iter().enumerate() {
            let mut row_iter = row.iter();

            if let Some(x) = row_iter.position(|&x| x != 0) {
                if row_iter.all(|&x| x == 0) {
                    let predicate = |row: &Vec<i32>| row[x] == 0;

                    if mat[..y].iter().all(predicate) && mat[y + 1..].iter().all(predicate) {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        Self::num_special(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

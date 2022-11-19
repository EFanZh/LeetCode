pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(mut upper: u32, mut lower: u32, colsum: Vec<i32>) -> Option<Vec<Vec<i32>>> {
        let mut row_0 = vec![0; colsum.len()];
        let mut row_1 = vec![0; colsum.len()];

        for ((top, bottom), sum) in row_0.iter_mut().zip(&mut row_1).zip(colsum) {
            match sum {
                0 => {}
                1 => {
                    if lower < upper {
                        *top = 1;
                        upper -= 1;
                    } else {
                        if lower == 0 {
                            return None;
                        }

                        *bottom = 1;
                        lower -= 1;
                    }
                }
                _ => {
                    if upper == 0 || lower == 0 {
                        return None;
                    }

                    *top = 1;
                    *bottom = 1;

                    upper -= 1;
                    lower -= 1;
                }
            }
        }

        (lower == 0 && upper == 0).then(|| vec![row_0, row_1])
    }

    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        Self::helper(upper as _, lower as _, colsum).unwrap_or_default()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        Self::reconstruct_matrix(upper, lower, colsum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

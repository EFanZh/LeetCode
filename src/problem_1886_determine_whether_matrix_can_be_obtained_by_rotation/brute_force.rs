pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check_0_degree(lhs: &[Vec<i32>], rhs: &[Vec<i32>]) -> bool {
        lhs == rhs
    }

    fn check_90_degrees(lhs: &[Vec<i32>], rhs: &[Vec<i32>]) -> bool {
        lhs.iter()
            .rev()
            .enumerate()
            .all(|(i, row)| row.iter().zip(rhs).all(|(&x, rhs_row)| x == rhs_row[i]))
    }

    fn check_180_degrees(lhs: &[Vec<i32>], rhs: &[Vec<i32>]) -> bool {
        lhs.iter()
            .zip(rhs.iter().rev())
            .all(|(lhs_row, rhs_row)| lhs_row.iter().zip(rhs_row.iter().rev()).all(|(x, y)| x == y))
    }

    fn check_270_degrees(lhs: &[Vec<i32>], rhs: &[Vec<i32>]) -> bool {
        lhs.iter().enumerate().all(|(i, lhs_row)| {
            lhs_row
                .iter()
                .zip(rhs.iter().rev())
                .all(|(&x, rhs_row)| x == rhs_row[i])
        })
    }

    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let lhs = mat.as_slice();
        let rhs = target.as_slice();

        Self::check_0_degree(lhs, rhs)
            || Self::check_90_degrees(lhs, rhs)
            || Self::check_180_degrees(lhs, rhs)
            || Self::check_270_degrees(lhs, rhs)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        Self::find_rotation(mat, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

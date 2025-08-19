pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        mat.iter().all(|row| {
            let n = row.len();

            row.iter().eq(row.iter().skip(k as u32 as usize % n).chain(row).take(n))
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        Self::are_similar(mat, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

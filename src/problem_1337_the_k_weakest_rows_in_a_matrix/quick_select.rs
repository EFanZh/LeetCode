pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut rows = (0_u32..)
            .zip(mat)
            .map(|(i, row)| (row.into_iter().sum::<i32>() as u32, i))
            .collect::<Vec<_>>();

        rows.select_nth_unstable(k - 1).0.sort_unstable();
        rows[..k].iter().map(|&(_, i)| i as i32).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        Self::k_weakest_rows(mat, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = u64::from(n as u32);

        (n * (n - 1) * 2 + 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn colored_cells(num: i32) -> i64 {
        Self::colored_cells(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

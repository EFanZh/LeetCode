pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m.cast_unsigned() as usize;
        let n = n.cast_unsigned() as usize;

        iter::once(".".repeat(n))
            .chain(iter::repeat_n(
                iter::repeat_n('#', n - 1).chain(iter::once('.')).collect(),
                m - 1,
            ))
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn create_grid(m: i32, n: i32) -> Vec<String> {
        Self::create_grid(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

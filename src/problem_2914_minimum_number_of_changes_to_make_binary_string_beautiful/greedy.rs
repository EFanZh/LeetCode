pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut result = 0;

        for &[x, y] in s.as_bytes().as_chunks::<2>().0 {
            result += i32::from(x != y);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_changes(s: String) -> i32 {
        Self::min_changes(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

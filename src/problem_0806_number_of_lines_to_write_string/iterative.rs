pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let max_width = 100_u8;
        let mut remaining = max_width;
        let mut lines = 1;

        for c in s.bytes() {
            let width = widths[usize::from(c) - usize::from(b'a')] as u8;

            if let Some(new_remaining) = remaining.checked_sub(width) {
                remaining = new_remaining;
            } else {
                lines += 1;
                remaining = max_width - width;
            }
        }

        vec![lines, (max_width - remaining).into()]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        Self::number_of_lines(widths, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

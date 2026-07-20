pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        s.as_bytes().windows(2).all(|window| window[0].abs_diff(window[1]) < 3)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_adjacent_diff_at_most_two(s: String) -> bool {
        Self::is_adjacent_diff_at_most_two(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

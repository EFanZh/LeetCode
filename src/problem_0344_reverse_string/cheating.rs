pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::ptr_arg, reason = "required")]
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_string(s: &mut Vec<char>) {
        Self::reverse_string(s);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

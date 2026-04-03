pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        (1..).zip(s.bytes()).map(|(i, c)| i * i32::from(b'z' + 1 - c)).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_degree(s: String) -> i32 {
        Self::reverse_degree(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

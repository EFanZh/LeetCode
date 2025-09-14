pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len();
        let quotient = n / 8;
        let remainder = n % 8;

        (4 * (1 + quotient) * quotient + (quotient + 1) * remainder) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_pushes(word: String) -> i32 {
        Self::minimum_pushes(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut result = 0;

        for byte in s.bytes() {
            result ^= byte;
        }

        for byte in t.bytes() {
            result ^= byte;
        }

        result.into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_difference(s: String, t: String) -> char {
        Self::find_the_difference(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

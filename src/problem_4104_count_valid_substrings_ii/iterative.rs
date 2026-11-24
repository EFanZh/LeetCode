pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_valid_substrings(s: String) -> i64 {
        let mut a = 0;
        let mut ab = 0;
        let mut abc = 0;
        let mut result = 0;

        (1..).zip(s.bytes()).for_each(|(i, c)| {
            match c {
                b'a' => a = i,
                b'b' => ab = a,
                _ => abc = ab,
            }

            result += i - abc;
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_valid_substrings(s: String) -> i64 {
        Self::count_valid_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

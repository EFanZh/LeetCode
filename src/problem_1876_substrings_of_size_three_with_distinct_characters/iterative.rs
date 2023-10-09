pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut iter = s.bytes();
        let mut first = iter.next().unwrap();
        let mut second = first;

        for c in iter {
            result += i32::from(first != second && first != c && second != c);
            first = second;
            second = c;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_substrings(s: String) -> i32 {
        Self::count_good_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(needle.as_str()).map_or(-1, |x| x as _)
    }
}

impl super::Solution for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        Self::str_str(haystack, needle)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

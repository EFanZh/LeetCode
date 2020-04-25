pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            0
        } else {
            haystack
                .as_bytes()
                .windows(needle.len())
                .position(|window| window == needle.as_bytes())
                .map_or(-1, |x| x as _)
        }
    }
}

impl super::Solution for Solution {
    fn str_str(haystack: String, needle: String) -> i32 {
        Self::str_str(haystack, needle)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}

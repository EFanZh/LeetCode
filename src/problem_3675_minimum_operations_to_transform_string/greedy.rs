pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        u32::from(s.bytes().filter(|&c| c != b'a').min().map_or(0, |c| b'z' + 1 - c)).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(s: String) -> i32 {
        Self::min_operations(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

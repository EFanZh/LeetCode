pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        s.bytes().fold(0_u32, |bits, c| bits | 1 << (c - b'a')).count_ones() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimized_string_length(s: String) -> i32 {
        Self::minimized_string_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

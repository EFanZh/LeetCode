pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_rotations(s: String) -> i32 {
        let mut prev = b'0';
        let mut result = 0;

        for c in s.bytes() {
            let diff = c.abs_diff(prev);

            result += i32::from(u8::min(diff, 10 - diff));
            prev = c;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_rotations(s: String) -> i32 {
        Self::min_rotations(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

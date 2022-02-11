pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut ones = 0;
        let mut min_flips = 0;

        for c in s.bytes() {
            if c == b'0' {
                min_flips = ones.min(min_flips + 1);
            } else {
                ones += 1;
            }
        }

        min_flips
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips_mono_incr(s: String) -> i32 {
        Self::min_flips_mono_incr(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

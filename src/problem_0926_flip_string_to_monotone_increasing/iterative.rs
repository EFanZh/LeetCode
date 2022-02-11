pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut min_flips_ends_with_1 = 0;
        let mut min_flips_ends_with_0 = 0;

        for c in s.bytes() {
            if c == b'0' {
                min_flips_ends_with_1 = min_flips_ends_with_0.min(min_flips_ends_with_1) + 1;
            } else {
                min_flips_ends_with_1 = min_flips_ends_with_0.min(min_flips_ends_with_1);
                min_flips_ends_with_0 += 1;
            }
        }

        min_flips_ends_with_0.min(min_flips_ends_with_1)
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

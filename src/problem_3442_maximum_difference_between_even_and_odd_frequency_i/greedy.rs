pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut counts = [0_u32; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut max_odd = 0;
        let mut min_even = u32::MAX;

        for count in counts {
            if count != 0 {
                if count.is_multiple_of(2) {
                    min_even = min_even.min(count);
                } else {
                    max_odd = max_odd.max(count);
                }
            }
        }

        (max_odd - min_even).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_difference(s: String) -> i32 {
        Self::max_difference(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

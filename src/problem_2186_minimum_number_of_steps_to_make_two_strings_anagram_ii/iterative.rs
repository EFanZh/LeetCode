pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut counts = [0_i32; 26];

        for c in s.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        for c in t.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] -= 1;
        }

        let mut result = 0;

        for count in counts {
            result += count.abs();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_steps(s: String, t: String) -> i32 {
        Self::min_steps(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

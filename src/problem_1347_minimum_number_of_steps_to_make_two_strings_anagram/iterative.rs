pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_letters(s: String) -> [u16; 26] {
        let mut result = [0; 26];

        for c in s.into_bytes() {
            result[usize::from(c) - usize::from(b'a')] += 1;
        }

        result
    }

    pub fn min_steps(s: String, t: String) -> i32 {
        let s_counts = Self::count_letters(s);
        let t_counts = Self::count_letters(t);
        let mut result = 0;

        for (&left, &right) in s_counts.iter().zip(&t_counts) {
            result += right.saturating_sub(left);
        }

        i32::from(result)
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

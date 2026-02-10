pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(mut lhs: u32, rhs: u32) -> u32 {
        const MODULUS: u32 = 1_000_000_007;

        lhs += rhs;

        lhs.checked_sub(MODULUS).unwrap_or(lhs)
    }

    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let t = t.cast_unsigned() as usize;
        let mut counts = [0_u32; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut cursor = 0_usize;

        for _ in 0..t {
            let next_cursor = cursor.checked_sub(1).unwrap_or(25);

            counts[cursor] = Self::add(counts[cursor], counts[next_cursor]);
            cursor = next_cursor;
        }

        counts.iter().copied().fold(0, Self::add).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_after_transformations(s: String, t: i32) -> i32 {
        Self::length_after_transformations(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

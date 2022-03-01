pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(Self::MODULUS).unwrap_or(result)
    }

    fn sub(lhs: u32, rhs: u32) -> u32 {
        lhs.checked_sub(rhs).unwrap_or(lhs + Self::MODULUS - rhs)
    }

    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut cache = [0; 26];
        let mut result = 0;

        for c in s.bytes() {
            let value = Self::add(result, 1);
            let old_value = mem::replace(&mut cache[usize::from(c) - usize::from(b'a')], value);

            result = Self::add(result, Self::sub(value, old_value));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_subseq_ii(s: String) -> i32 {
        Self::distinct_subseq_ii(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

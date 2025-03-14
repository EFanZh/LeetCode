pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn assign_add(target: &mut u32, value: u32) {
        *target += value;
        *target = target.checked_sub(Self::MODULUS).unwrap_or(*target);
    }

    fn add_subsequence(counts: &mut ([u32; 10], [u32; 100]), c: u8) {
        let digit = usize::from(c) - usize::from(b'0');

        assert!(digit < 10);

        counts.1[10 * digit..10 * digit + 10]
            .iter_mut()
            .zip(counts.0)
            .for_each(|(count_2, count_1)| Self::assign_add(count_2, count_1));

        counts.0[digit] += 1;
    }

    pub fn count_palindromes(s: String) -> i32 {
        let mut prev = ([0_u32; 10], [0_u32; 100]);

        let left_counts = s
            .bytes()
            .map(|c| {
                let result = prev;

                Self::add_subsequence(&mut prev, c);

                result
            })
            .collect::<Box<_>>();

        prev = ([0_u32; 10], [0_u32; 100]);

        let mut result = 0;

        s.bytes().zip(&*left_counts).rev().for_each(|(c, left_counts)| {
            left_counts
                .1
                .iter()
                .zip(&prev.1)
                .for_each(|(&lhs, &rhs)| result += (u64::from(lhs) * u64::from(rhs)) % u64::from(Self::MODULUS));

            Self::add_subsequence(&mut prev, c);
        });

        (result % u64::from(Self::MODULUS)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_palindromes(s: String) -> i32 {
        Self::count_palindromes(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

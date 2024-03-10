pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut s = s.into_bytes();

        for c in &mut s {
            *c -= b'0';
        }

        let n = s.len() as u32;
        let odd_length = n / 2;
        let even_length = n - odd_length;
        let mut even_ones = s.iter().copied().step_by(2).map(u32::from).sum::<u32>();
        let mut odd_ones = s[1..].iter().copied().step_by(2).map(u32::from).sum::<u32>();
        let mut result = (even_ones + (odd_length - odd_ones)).min(even_length - even_ones + odd_ones);

        if even_length != odd_length {
            for c in s {
                let c = u32::from(c);

                (even_ones, odd_ones) = (odd_ones + c, even_ones - c);

                result = result
                    .min(even_ones + (odd_length - odd_ones))
                    .min(even_length - even_ones + odd_ones);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(s: String) -> i32 {
        Self::min_flips(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

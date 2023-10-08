pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn fold_count(count: u16, c: u8) -> u16 {
        count + u16::from(c) - u16::from(b'0')
    }

    pub fn min_swaps(s: String) -> i32 {
        let n = s.len() as u16;
        let ones = s.bytes().fold(0, Self::fold_count);
        let zeros = n - ones;
        let diff_plus_1 = (ones + 1).wrapping_sub(zeros);

        if diff_plus_1 < 3 {
            let even_ones = s.bytes().step_by(2).fold(0, Self::fold_count);

            i32::from(match diff_plus_1 {
                0 => even_ones,
                1 => even_ones.min((n + 1) / 2 - even_ones),
                _ => (n + 1) / 2 - even_ones,
            })
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(s: String) -> i32 {
        Self::min_swaps(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

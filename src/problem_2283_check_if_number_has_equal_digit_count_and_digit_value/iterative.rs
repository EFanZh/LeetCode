pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut num = num.into_bytes();
        let mut counts = [0_u8; 10];

        for c in &mut num {
            *c -= b'0';

            counts[usize::from(*c)] += 1;
        }

        counts.iter().zip(&num).all(|(actual, expected)| actual == expected)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn digit_count(num: String) -> bool {
        Self::digit_count(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

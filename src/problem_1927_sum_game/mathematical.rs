pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn weight(s: &[u8]) -> u32 {
        let mut sum = 0;
        let mut placeholders = 0;

        for &c in s {
            if c == b'?' {
                placeholders += 1;
            } else {
                sum += u32::from(c - b'0');
            }
        }

        sum * 2 + placeholders * 9
    }

    pub fn sum_game(num: String) -> bool {
        let (left, right) = num.as_bytes().split_at(num.len() / 2);

        Self::weight(left) != Self::weight(right)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_game(num: String) -> bool {
        Self::sum_game(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

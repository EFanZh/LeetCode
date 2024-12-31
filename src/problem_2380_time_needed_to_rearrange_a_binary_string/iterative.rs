pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut zeroes = 0;
        let mut cost = 0_u16;

        s.bytes().skip_while(|&c| c == b'1').for_each(|c| {
            if c == b'0' {
                zeroes += 1;
            } else {
                cost = zeroes.max(cost + 1);
            }
        });

        i32::from(cost)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn seconds_to_remove_occurrences(s: String) -> i32 {
        Self::seconds_to_remove_occurrences(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

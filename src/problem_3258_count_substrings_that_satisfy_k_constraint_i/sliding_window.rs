pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let k = k.cast_unsigned();
        let mut zeros = 0;
        let mut ones = 0;

        (1..)
            .zip(s.bytes())
            .map(|(length, c)| {
                if c == b'0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }

                while zeros as u32 > k && ones as u32 > k {
                    if s.as_bytes()[length - (zeros + ones)] == b'0' {
                        zeros -= 1;
                    } else {
                        ones -= 1;
                    }
                }

                zeros as i32 + ones as i32
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        Self::count_k_constraint_substrings(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn password_strength(password: String) -> i32 {
        let mut lower_cases = 0_u32;
        let mut upper_cases = 0_u32;
        let mut digits = 0_u16;
        let mut specials = 0_u8;

        for c in password.into_bytes() {
            match c {
                b'a'..=b'z' => lower_cases |= 1 << (c - b'a'),
                b'A'..=b'Z' => upper_cases |= 1 << (c - b'A'),
                b'0'..=b'9' => digits |= 1 << (c - b'0'),
                b'!' => specials |= 1,
                b'#' => specials |= 2,
                b'$' => specials |= 4,
                _ => specials |= 8,
            }
        }

        (lower_cases.count_ones() + upper_cases.count_ones() * 2 + digits.count_ones() * 3 + specials.count_ones() * 5)
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn password_strength(password: String) -> i32 {
        Self::password_strength(password)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

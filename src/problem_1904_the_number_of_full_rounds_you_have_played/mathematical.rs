pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_time(time: &str) -> u16 {
        let [h0, h1, _, m0, m1] = time.as_bytes().try_into().ok().unwrap();

        u16::from(h0) * 600 + u16::from(h1) * 60 + u16::from(m0) * 10 + u16::from(m1)
            - u16::from(b'0') * (600 + 60 + 10 + 1)
    }

    pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        let login_time = Self::parse_time(&login_time);
        let mut logout_time = Self::parse_time(&logout_time);

        if logout_time < login_time {
            logout_time += 1440;
        }

        i32::from((logout_time / 15).saturating_sub(login_time.div_ceil(15)))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
        Self::number_of_rounds(login_time, logout_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

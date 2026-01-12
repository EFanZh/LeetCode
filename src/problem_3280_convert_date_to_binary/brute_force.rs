pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let [y0, y1, y2, y3, _, m0, m1, _, d0, d1] = date.into_bytes().try_into().ok().unwrap();

        format!(
            "{:b}-{:b}-{:b}",
            u16::from(y0) * 1000 + u16::from(y1) * 100 + u16::from(y2) * 10 + u16::from(y3) - u16::from(b'0') * 1111,
            u16::from(m0) * 10 + u16::from(m1) - u16::from(b'0') * 11,
            u16::from(d0) * 10 + u16::from(d1) - u16::from(b'0') * 11,
        )
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn convert_date_to_binary(date: String) -> String {
        Self::convert_date_to_binary(date)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn parse_month(m1: u8, m2: u8) -> [u8; 2] {
        match m2 {
            b'b' => *b"02",
            b'c' => *b"12",
            b'g' => *b"08",
            b'l' => *b"07",
            b'n' => match m1 {
                b'a' => *b"01",
                _ => *b"06",
            },
            b'p' => *b"09",
            b'r' => match m1 {
                b'a' => *b"03",
                _ => *b"04",
            },
            b't' => *b"10",
            b'v' => *b"11",
            _ => *b"05",
        }
    }

    pub fn reformat_date(date: String) -> String {
        let mut date = date.into_bytes();
        let s = date.as_slice();

        let ([y0, y1, y2, y3], [m1, m2], [d0, d1]) = if let [d, _, _, _, _, m1, m2, _, y0, y1, y2, y3] = *s {
            ([y0, y1, y2, y3], [m1, m2], [b'0', d])
        } else {
            let [d0, d1, _, _, _, _, m1, m2, _, y0, y1, y2, y3]: [_; 13] = s.try_into().unwrap();

            ([y0, y1, y2, y3], [m1, m2], [d0, d1])
        };

        let [m0, m1] = Self::parse_month(m1, m2);

        date.clear();
        date.extend([y0, y1, y2, y3, b'-', m0, m1, b'-', d0, d1]);

        String::from_utf8(date).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reformat_date(date: String) -> String {
        Self::reformat_date(date)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

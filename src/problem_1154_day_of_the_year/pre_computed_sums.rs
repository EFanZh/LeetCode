pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        static COMMON_YEAR_DAYS: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        static LEAP_YEAR_DAYS: [i32; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

        let [y1, y2, y3, y4, _, m1, m2, _, d1, d2]: [_; 10] = date.into_bytes().try_into().unwrap();

        let year =
            1000 * u16::from(y1) + 100 * u16::from(y2) + 10 * u16::from(y3) + u16::from(y4) - (1111 * u16::from(b'0'));

        let month = 10 * usize::from(m1) + usize::from(m2) - (11 * usize::from(b'0') + 1);
        let day = 10 * i32::from(d1) + i32::from(d2) - 11 * i32::from(b'0');

        if year % (if year % 100 == 0 { 400 } else { 4 }) == 0 {
            LEAP_YEAR_DAYS[month] + day
        } else {
            COMMON_YEAR_DAYS[month] + day
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn day_of_year(date: String) -> i32 {
        Self::day_of_year(date)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

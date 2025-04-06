pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_date(date: &str) -> u32 {
        static COMMON_YEAR_DAYS: [u32; 11] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304];
        static LEAP_YEAR_DAYS: [u32; 11] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305];

        let [y1, y2, y3, y4, _, m1, m2, _, d1, d2] = date.as_bytes().try_into().ok().unwrap();

        let year =
            1000 * u32::from(y1) + 100 * u32::from(y2) + 10 * u32::from(y3) + u32::from(y4) - u32::from(b'0') * 1111;

        let month = 10 * u32::from(m1) + u32::from(m2) - (u32::from(b'0') * 11 + 1);
        let day = 10 * u32::from(d1) + u32::from(d2) - (u32::from(b'0') * 11 + 1);
        let leap_years_before_this_year = year.div_ceil(4) - year.div_ceil(100) + year.div_ceil(400);
        let days_before_this_year = 365 * year + leap_years_before_this_year;
        let is_this_year_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

        let days_before_this_month = if is_this_year_leap_year {
            LEAP_YEAR_DAYS.get(month as usize).copied().unwrap_or(335)
        } else {
            COMMON_YEAR_DAYS.get(month as usize).copied().unwrap_or(334)
        };

        days_before_this_year + days_before_this_month + day
    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let date1 = Self::parse_date(&date1);
        let date2 = Self::parse_date(&date2);

        (if date2 < date1 { date1 - date2 } else { date2 - date1 }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn days_between_dates(date1: String, date2: String) -> i32 {
        Self::days_between_dates(date1, date2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

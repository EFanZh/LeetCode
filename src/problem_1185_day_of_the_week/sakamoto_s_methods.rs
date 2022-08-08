pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week#Sakamoto's_methods>.

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        const WEEK_DAYS: [&str; 7] = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ];

        const MONTH_DAYS: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

        let day = day as u32;
        let month = month as u32;
        let mut year = year as u32;

        year -= u32::from(month < 3);

        let week_day = (year + (year / 4 - year / 100 + year / 400) + MONTH_DAYS[month as usize - 1] + day) % 7;

        WEEK_DAYS[week_day as usize].to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        Self::day_of_the_week(day, month, year)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

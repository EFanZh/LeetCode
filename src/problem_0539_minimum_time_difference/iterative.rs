pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    fn parse_time(s: &str) -> u16 {
        let [h1, h2, _, m1, m2]: [u8; 5] = s.as_bytes().try_into().unwrap();

        600 * u16::from(h1) + 60 * u16::from(h2) + 10 * u16::from(m1) + u16::from(m2) - 671 * u16::from(b'0')
    }

    pub fn find_min_difference(mut time_points: Vec<String>) -> i32 {
        time_points.sort_unstable();

        let mut result = u16::MAX;
        let (first, rest) = time_points.split_first().unwrap();
        let first_time = Self::parse_time(first);
        let mut prev = first_time;

        for time in rest {
            let time = Self::parse_time(time);

            result = result.min(time - prev);
            prev = time;
        }

        result.min(first_time + (1440 - prev)).into()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_min_difference(time_points: Vec<String>) -> i32 {
        Self::find_min_difference(time_points)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

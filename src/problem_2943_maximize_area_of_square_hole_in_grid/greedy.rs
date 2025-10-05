pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn calculate_longest_consecutive_bars(bars: &[u32]) -> u32 {
        let mut prev = u32::MAX;
        let mut result = 0;
        let mut length = 0;

        for &bar in bars {
            if bar == prev.wrapping_add(1) {
                length += 1;
            } else {
                result = result.max(length);
                length = 1;
            }

            prev = bar;
        }

        result.max(length)
    }

    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        _ = (m, n);

        let [mut h_bars, mut v_bars] =
            [h_bars, v_bars].map(|bars| bars.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>());

        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let h_consecutive_bars = Self::calculate_longest_consecutive_bars(&h_bars);
        let v_consecutive_bars = Self::calculate_longest_consecutive_bars(&v_bars);

        (h_consecutive_bars.min(v_consecutive_bars) + 1).pow(2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        Self::maximize_square_hole_area(n, m, h_bars, v_bars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

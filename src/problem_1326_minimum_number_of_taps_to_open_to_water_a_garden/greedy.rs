pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut ranges = (0..)
            .zip(ranges)
            .map(|(i, range)| (i - range, i + range))
            .collect::<Vec<_>>();

        ranges.sort_unstable_by_key(|&(start, _)| start);

        let mut max_end_1 = 0;
        let mut max_end_2 = 0;
        let mut result = 0;

        for (start, end) in ranges {
            if start > max_end_2 {
                return -1;
            }

            if start > max_end_1 {
                max_end_1 = max_end_2;
                result += 1;
            }

            max_end_2 = max_end_2.max(end);
        }

        if n <= max_end_1 {
            result
        } else {
            result + 1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        Self::min_taps(n, ranges)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

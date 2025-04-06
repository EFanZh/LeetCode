pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let count: &[_; 256] = count.as_slice().try_into().unwrap();
        let make_iter = || count.iter().zip(0_u32..).map(|(&count, value)| (value, count as u32));

        // Minimum.

        let min = count.iter().position(|&value| value != 0).unwrap() as u32;

        // Maximum.

        let max = count.iter().rposition(|&value| value != 0).unwrap() as u32;

        // Mean and mode.

        let mut sum = 0_u64;
        let mut total_count = 0_u32;
        let mut mode = 0;
        let mut max_count = 0_u32;

        for (value, count) in make_iter() {
            sum += u64::from(value) * u64::from(count);
            total_count += count;

            if count > max_count {
                mode = value;
                max_count = count;
            }
        }

        #[expect(clippy::cast_precision_loss, reason = "optimal")]
        let mean = (sum as f64) / f64::from(total_count);

        // Median.

        let mut current = (0, 0);
        let mut iter = make_iter();

        let mut next = |mut n: u32| {
            while n != 0 {
                while current.1 == 0 {
                    current = iter.next().unwrap();
                }

                if let Some(next_n) = n.checked_sub(current.1) {
                    n = next_n;
                    current.1 = 0;
                } else {
                    current.1 -= n;

                    break;
                }
            }

            current.0
        };

        let left = next(total_count.div_ceil(2));

        let median = if total_count % 2 == 0 {
            f64::from(left + next(1)) / 2.0
        } else {
            f64::from(left)
        };

        vec![f64::from(min), f64::from(max), mean, median, f64::from(mode)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        Self::sample_stats(count)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

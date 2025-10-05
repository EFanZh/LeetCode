pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn partition(values: &mut [u32], key: u32) -> usize {
        // +------+-------+-----------+---------+
        // | Less | Equal | Unchecked | Greater |
        // +------+-------+-----------+---------+

        let mut equal_start = 0;
        let mut unchecked_start = 0;
        let mut greater_start = values.len();

        while unchecked_start < greater_start {
            match values[unchecked_start].cmp(&key) {
                Ordering::Less => {
                    values.swap(equal_start, unchecked_start);

                    equal_start += 1;
                    unchecked_start += 1;
                }
                Ordering::Equal => unchecked_start += 1,
                Ordering::Greater => {
                    greater_start -= 1;

                    values.swap(unchecked_start, greater_start);
                }
            }
        }

        (values.len() / 2).clamp(equal_start, unchecked_start - 1)
    }

    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let n = happiness.len();
        let k = k.cast_unsigned() as usize;

        happiness.select_nth_unstable(n - k);

        let mut window = &mut happiness[n - k..];
        let mut right_length = 0;

        while !window.is_empty() {
            let key = window[window.len() / 2];
            let middle = Self::partition(window, key);

            let candidate_right_length = window.len() - middle + right_length;

            window = if window[middle] < candidate_right_length as u32 {
                &mut window[middle + 1..]
            } else {
                right_length = candidate_right_length;

                &mut window[..middle]
            };
        }

        let selected = &happiness[n - right_length..];
        let selected_len = selected.len() as u64;

        let sum = selected
            .iter()
            .fold(0_u64, |sum, &happiness| sum + u64::from(happiness));

        let penalty = selected_len * (selected_len - 1) / 2;

        (sum - penalty).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        Self::maximum_happiness_sum(happiness, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

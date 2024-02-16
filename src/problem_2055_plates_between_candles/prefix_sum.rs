pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sums = vec![(0_u32, 0_u32); s.len()].into_boxed_slice();
        let mut full_plates = 0;
        let mut current_plates = 0;

        for (target, c) in prefix_sums.iter_mut().zip(s.bytes()) {
            if c == b'|' {
                full_plates += current_plates;
                current_plates = 0;
            } else {
                current_plates += 1;
            }

            target.1 = full_plates;
        }

        full_plates += current_plates;
        current_plates = 0;

        for (target, c) in prefix_sums.iter_mut().zip(s.bytes()).rev() {
            if c == b'|' {
                full_plates -= current_plates;
                current_plates = 0;
            } else {
                current_plates += 1;
            }

            target.0 = full_plates;
        }

        queries
            .into_iter()
            .map(|query| {
                let [left, right]: [_; 2] = query.try_into().ok().unwrap();
                let right_sum = prefix_sums[right as u32 as usize].1;
                let left_sum = prefix_sums[left as u32 as usize].0;

                right_sum.saturating_sub(left_sum) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::plates_between_candles(s, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

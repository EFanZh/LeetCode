pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn min_max(values: &[i32]) -> (u32, u32) {
        let mut iter = values.iter().map(|&num| num as u32);
        let min = iter.next().unwrap();
        let mut result = (min, min);

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next() {
                if right < left {
                    result.0 = result.0.min(right);
                    result.1 = result.1.max(left);
                } else {
                    result.0 = result.0.min(left);
                    result.1 = result.1.max(right);
                }
            } else {
                if left < result.0 {
                    result.0 = left;
                } else if left > result.1 {
                    result.1 = left;
                }

                break;
            }
        }

        result
    }

    fn inner(bloom_day: &[i32], m: u32, k: NonZeroU32) -> u32 {
        if bloom_day.len() as u32 / k < m {
            u32::MAX
        } else {
            let check = |middle: u32| {
                let mut length = 0;
                let mut bouquets = 0;

                for &day in bloom_day {
                    if day as u32 <= middle {
                        length += 1;
                    } else {
                        bouquets += length / k;

                        if bouquets >= m {
                            return true;
                        }

                        length = 0;
                    }
                }

                bouquets += length / k;
                bouquets >= m
            };

            let (mut left, mut right) = Self::min_max(bloom_day);

            while left < right {
                let middle = (left + right) / 2;

                if check(middle) {
                    right = middle;
                } else {
                    left = middle + 1;
                }
            }

            left
        }
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        Self::inner(&bloom_day, m as _, NonZeroU32::new(k as _).unwrap()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        Self::min_days(bloom_day, m, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

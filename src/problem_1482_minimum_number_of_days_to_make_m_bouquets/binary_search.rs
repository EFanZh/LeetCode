pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn inner(bloom_day: &[i32], m: u32, k: NonZeroU32) -> u32 {
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

        let mut start = 1;
        let mut count = u32::MAX - 1;

        loop {
            let half = count / 2;
            let middle = start + half;

            if check(middle) {
                count = half;
            } else {
                start = middle + 1;
                count -= half + 1;
            }

            if count == 0 {
                break;
            }
        }

        start
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

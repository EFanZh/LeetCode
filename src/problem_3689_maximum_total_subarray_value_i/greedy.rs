pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn min_max(nums: &[u32]) -> Option<(u32, u32)> {
        let (chunks, suffix) = nums.as_chunks();
        let mut iter = chunks.iter().copied();

        let mut result = iter.next().map(|[mut min, mut max]| {
            for &[mut x, mut y] in chunks {
                if y < x {
                    mem::swap(&mut x, &mut y);
                }

                if x < min {
                    min = x;
                }

                if y > max {
                    max = y;
                }
            }

            (min, max)
        });

        if let &[last] = suffix {
            if let Some((min, max)) = &mut result {
                if last < *min {
                    *min = last;
                } else if last > *max {
                    *max = last;
                }
            } else {
                result = Some((last, last));
            }
        }

        result
    }

    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let (min, max) = Self::min_max(&nums).unwrap();

        (u64::from(max - min) * u64::from(k.cast_unsigned())).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        Self::max_total_value(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn shrink(k: NonZero<u32>, mut length: u32, remainder: u32, mut sum_remainder: u32) -> u32 {
        while remainder != sum_remainder {
            if sum_remainder < remainder {
                sum_remainder += k.get();
            }

            sum_remainder -= remainder;
            length -= 1;
        }

        length
    }

    pub fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = NonZero::<u32>::new(k.cast_unsigned()).unwrap();
        let mut iter = nums.iter().copied();
        let mut length = 1;
        let mut remainder = iter.next().unwrap() % k;
        let mut sum_remainder = remainder;
        let mut result = 1_u32;

        for num in iter {
            let r = num % k;

            if r == remainder {
                length += 1;
                sum_remainder += r;

                if sum_remainder >= k.get() {
                    sum_remainder -= k.get();
                }
            } else {
                if length > result {
                    result = result.max(Self::shrink(k, length, remainder, sum_remainder));
                }

                length = 1;
                remainder = r;
                sum_remainder = r;
            }
        }

        if length > result {
            result = result.max(Self::shrink(k, length, remainder, sum_remainder));
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn resilient_subarray(nums: Vec<i32>, k: i32) -> i32 {
        Self::resilient_subarray(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

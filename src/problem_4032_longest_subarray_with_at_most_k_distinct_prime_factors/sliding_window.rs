pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn factorize(smallest_prime_factors: &[NonZero<u32>], mut num: u32, mut f: impl FnMut(NonZero<u32>)) {
        loop {
            let factor = smallest_prime_factors[num as usize - 2];

            f(factor);

            if factor.get() == num {
                break;
            }

            num /= factor;
        }
    }

    pub fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();
        let max = nums.iter().copied().fold(0, u32::max);
        let mut smallest_prime_factors = vec![NonZero::<u32>::MAX; (max - 1) as usize].into_boxed_slice();
        let mut max_prime_factor = NonZero::<u32>::MIN;

        for factor in NonZero::new(2).unwrap()..=NonZero::new(max).unwrap() {
            let factor_usize = factor.get() as usize;

            let mut iter = smallest_prime_factors[factor_usize - 2..]
                .iter_mut()
                .step_by(factor_usize);

            let first = iter.next().unwrap();

            if *first == NonZero::<u32>::MAX {
                *first = factor;
                max_prime_factor = factor;

                iter.for_each(|target| {
                    if *target == NonZero::<u32>::MAX {
                        *target = factor;
                    }
                });
            }
        }

        let mut start = 0;
        let mut counts = vec![0; (max_prime_factor.get() - 1) as usize];
        let mut unique_count = 0;

        for &num in &nums {
            Self::factorize(&smallest_prime_factors, num, |factor| {
                let count = &mut counts[(factor.get() - 2) as usize];

                unique_count += u32::from(*count == 0);

                *count += 1;
            });

            if unique_count > k {
                Self::factorize(&smallest_prime_factors, nums[start], |factor| {
                    let count = &mut counts[(factor.get() - 2) as usize];

                    *count -= 1;
                    unique_count -= u32::from(*count == 0);
                });

                start += 1;
            }
        }

        (nums.len() - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        Self::longest_subarray(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

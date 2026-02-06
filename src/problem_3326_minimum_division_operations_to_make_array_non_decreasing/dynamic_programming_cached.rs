pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::sync::OnceLock;

const N: usize = 1_000_000;

impl Solution {
    #[expect(clippy::unnecessary_box_returns, reason = "by design")]
    fn zeroed_array<const N: usize>() -> Box<[u16; N]> {
        Box::try_from(vec![0; N].into_boxed_slice()).unwrap()
    }

    pub fn get_min_factors() -> &'static [u16; N + 1] {
        static FACTORS: OnceLock<Box<[u16; N + 1]>> = OnceLock::new();

        FACTORS.get_or_init(|| {
            const MIDDLE: usize = N.isqrt();

            let mut result = Self::zeroed_array::<{ N + 1 }>();

            for factor in 2..=MIDDLE {
                if result[factor] == 0 {
                    result
                        .iter_mut()
                        .skip(factor * factor)
                        .step_by(factor)
                        .for_each(|target| {
                            if *target == 0 {
                                *target = factor as _;
                            }
                        });
                }
            }

            result
        })
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let min_factors = Self::get_min_factors();
        let mut low = 0;
        let mut low_cost = 0;
        let mut high = 0;
        let mut high_cost = 0;

        for num in nums {
            let new_high_cost = if high <= num {
                high_cost
            } else if low <= num {
                low_cost
            } else {
                return -1;
            };

            let min_factor = u32::from(min_factors[num as usize]);

            let (new_low, new_low_cost) = 'outer: {
                'inner: {
                    if min_factor != 0 {
                        let new_low_cost = (if high <= min_factor {
                            high_cost
                        } else if low <= min_factor {
                            low_cost
                        } else {
                            break 'inner;
                        }) + 1;

                        break 'outer (min_factor, new_low_cost);
                    }

                    break 'inner;
                }

                (num, new_high_cost)
            };

            (low, low_cost, high, high_cost) = (new_low, new_low_cost, num, new_high_cost);
        }

        high_cost
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        Self::min_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

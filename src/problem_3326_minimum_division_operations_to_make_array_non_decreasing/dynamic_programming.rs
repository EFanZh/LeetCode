pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_min_factors(n: u32) -> Box<[u16]> {
        let n = n as usize;
        let mut result = vec![0; n + 1].into_boxed_slice();
        let middle = n.isqrt();

        assert!(middle < result.len());

        for factor in 2..=middle {
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
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let max = nums.iter().copied().fold(0, u32::max);
        let min_factors = Self::get_min_factors(max);
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

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(num: u32) -> u32 {
        let sqrt_num = num.isqrt();

        if sqrt_num * sqrt_num == num {
            0
        } else {
            let mut i = 2;

            let (second, third) = loop {
                if i <= sqrt_num {
                    if num.is_multiple_of(i) {
                        break (i, num / i);
                    }

                    i += 1;
                } else {
                    return 0;
                }
            };

            loop {
                i += 1;

                if i <= sqrt_num {
                    if num.is_multiple_of(i) {
                        return 0;
                    }
                } else {
                    return 1 + second + third + num;
                }
            }
        }
    }

    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for num in nums {
            result += Self::check(num as _);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        Self::sum_four_divisors(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

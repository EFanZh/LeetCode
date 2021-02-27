pub struct Solution;

use std::iter;

impl Solution {
    fn divide_negative(mut dividend: i32, divisor: i32) -> i32 {
        let mut result = 0;

        while dividend <= divisor {
            dividend -= divisor;
            result -= 1;

            for (x, i) in iter::successors(divisor.checked_add(divisor).map(|x| (x, -2)), |(x, i)| {
                x.checked_add(*x).map(|x| (x, i + i))
            }) {
                if dividend <= x {
                    dividend -= x;
                    result += i;
                } else {
                    break;
                }
            }
        }

        result
    }

    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::min_value() && divisor == -1 {
            i32::max_value()
        } else if dividend < 0 {
            if divisor < 0 {
                -Self::divide_negative(dividend, divisor)
            } else {
                Self::divide_negative(dividend, -divisor)
            }
        } else if divisor < 0 {
            Self::divide_negative(-dividend, divisor)
        } else {
            -Self::divide_negative(-dividend, -divisor)
        }
    }
}

impl super::Solution for Solution {
    fn divide(dividend: i32, divisor: i32) -> i32 {
        Self::divide(dividend, divisor)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

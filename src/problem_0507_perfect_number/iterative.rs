pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::if_then_some_else_none)]
    pub fn check_perfect_number(num: i32) -> bool {
        let num = num as u32;
        let last = f64::from(num).sqrt() as u32;

        let divisor_sum = 1
            + (2..last)
                .filter_map(|x| if num % x == 0 { Some(x + num / x) } else { None })
                .sum::<u32>();

        if num % last == 0 {
            if last * last == num {
                num == divisor_sum + last
            } else {
                num == divisor_sum + last + num / last
            }
        } else {
            num == divisor_sum
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_perfect_number(num: i32) -> bool {
        Self::check_perfect_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

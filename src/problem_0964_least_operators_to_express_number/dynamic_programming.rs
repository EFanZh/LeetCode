pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let x = NonZeroU32::new(x as _).unwrap();
        let mut target = target as u32;
        let digit = target % x;
        let mut plus = 2 * digit;
        let mut minus = 2 * (x.get() - digit);
        let mut base = 1;

        loop {
            target = target / x;

            if target == 0 {
                break;
            }

            let digit = target % x;
            let digit_2 = x.get() - digit;
            let new_plus = (base * digit + plus).min(base * (digit + 1) + minus);
            let new_minus = (base * digit_2 + plus).min(base * (digit_2 - 1) + minus);

            plus = new_plus;
            minus = new_minus;
            base += 1;
        }

        (plus.min(base + minus) - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn least_ops_express_target(x: i32, target: i32) -> i32 {
        Self::least_ops_express_target(x, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

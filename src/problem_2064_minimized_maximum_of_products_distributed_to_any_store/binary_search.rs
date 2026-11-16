pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn check(mut n: u32, quantities: &[i32], limit: NonZeroU32) -> bool {
        for &quantity in quantities {
            let required = (quantity.cast_unsigned() + limit.get() - 1) / limit;

            if n < required {
                return false;
            }

            n -= required;
        }

        true
    }

    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let n = n.cast_unsigned();
        let quantities = quantities.as_slice();
        let mut left = 1;
        let mut right = quantities.iter().copied().map(i32::cast_unsigned).max().unwrap();

        while left < right {
            let middle = u32::midpoint(left, right);

            if Self::check(n, quantities, NonZeroU32::new(middle).unwrap()) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        Self::minimized_maximum(n, quantities)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

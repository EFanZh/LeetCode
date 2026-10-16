pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let num = num.cast_unsigned();
        let sum = sum.cast_unsigned();
        let nines = sum / 9;
        let remainder = sum % 9;
        let digits = nines + u32::from(remainder != 0);

        if digits <= num {
            String::from_utf8(
                iter::repeat_n(b'9', nines as usize)
                    .chain((remainder != 0).then_some(b'0' + remainder as u8))
                    .chain(iter::repeat_n(b'0', (num - digits) as usize))
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        } else {
            String::new()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_of_squares(num: i32, sum: i32) -> String {
        Self::max_sum_of_squares(num, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

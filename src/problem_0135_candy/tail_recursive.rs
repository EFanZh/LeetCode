pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn triangular(x: i32) -> i32 {
        x * (x + 1) / 2
    }

    fn triangular_2(x: i32, y: i32) -> i32 {
        (x * (x + 1) + y * (y + 1)) / 2
    }

    fn going_down(first: i32, rest: &[i32], up_length: i32, down_length: i32, result: &mut i32) {
        if let Some((&second, rest)) = rest.split_first() {
            match second.cmp(&first) {
                Ordering::Less => Self::going_down(second, rest, up_length, down_length + 1, result),
                Ordering::Equal => {
                    *result += Self::triangular_2(up_length, down_length) + up_length.max(down_length) + 1;

                    Self::going_up(second, rest, 0, result);
                }
                Ordering::Greater => {
                    *result += Self::triangular_2(up_length, down_length) + up_length.max(down_length);

                    Self::going_up(second, rest, 1, result);
                }
            }
        } else {
            *result += Self::triangular_2(up_length, down_length) + up_length.max(down_length) + 1;
        }
    }

    fn going_up(first: i32, rest: &[i32], up_length: i32, result: &mut i32) {
        if let Some((&second, rest)) = rest.split_first() {
            match second.cmp(&first) {
                Ordering::Less => Self::going_down(second, rest, up_length, 1, result),
                Ordering::Equal => {
                    *result += Self::triangular(up_length) + up_length + 1;

                    Self::going_up(second, rest, 0, result);
                }
                Ordering::Greater => Self::going_up(second, rest, up_length + 1, result),
            }
        } else {
            *result += Self::triangular(up_length) + up_length + 1;
        }
    }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut result = 0;

        if let Some((first, rest)) = ratings.split_first() {
            Self::going_up(*first, rest, 0, &mut result);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn candy(ratings: Vec<i32>) -> i32 {
        Self::candy(ratings)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

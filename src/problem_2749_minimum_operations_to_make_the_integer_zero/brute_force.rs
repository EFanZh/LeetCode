pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = i64::from(num1);
        let num2 = i64::from(num2);
        let mut result = 0;

        loop {
            let extra = num1 - num2 * result;

            if extra < result {
                return -1;
            }

            if (result as u32) < extra.count_ones() {
                result += 1;
            } else {
                break;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        Self::make_the_integer_zero(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

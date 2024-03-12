pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let num1 = num1 as u32;
        let num2 = num2 as u32;
        let (mut num1, mut num2) = if num2 < num1 { (num2, num1) } else { (num1, num2) };
        let mut result = 0;

        while let Some(non_zero_num1) = NonZeroU32::new(num1) {
            result += num2 / non_zero_num1;

            let new_num1 = num2 % non_zero_num1;

            num2 = num1;
            num1 = new_num1;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_operations(num1: i32, num2: i32) -> i32 {
        Self::count_operations(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let x = x as u8;
        let mut sum = 0;
        let mut n = x;

        while n != 0 {
            (sum, n) = (sum + n % 10, n / 10);
        }

        if x.is_multiple_of(sum) { i32::from(sum) } else { -1 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        Self::sum_of_the_digits_of_harshad_number(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

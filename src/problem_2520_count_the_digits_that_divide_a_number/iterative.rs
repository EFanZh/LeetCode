pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let num = num as u32;
        let mut x = num;
        let mut result = 0;

        while x != 0 {
            result += u8::from(num.is_multiple_of(x % 10));
            x /= 10;
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_digits(num: i32) -> i32 {
        Self::count_digits(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

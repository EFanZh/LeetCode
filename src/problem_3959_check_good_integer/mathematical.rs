pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_good_integer(n: i32) -> bool {
        let mut diff = 0;
        let mut n = n.cast_unsigned();

        while n != 0 {
            let digit = n % 10;

            n /= 10;
            diff += digit * digit.wrapping_sub(1);
        }

        diff >= 50
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_good_integer(n: i32) -> bool {
        Self::check_good_integer(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

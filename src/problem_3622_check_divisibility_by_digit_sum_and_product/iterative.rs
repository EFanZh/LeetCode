pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let n = n.cast_unsigned();
        let mut sum = 0;
        let mut product = 1;
        let mut x = n;

        loop {
            let digit = x % 10;

            x /= 10;

            sum += digit;
            product *= digit;

            if x == 0 {
                break;
            }
        }

        n.is_multiple_of(sum + product)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_divisibility(n: i32) -> bool {
        Self::check_divisibility(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

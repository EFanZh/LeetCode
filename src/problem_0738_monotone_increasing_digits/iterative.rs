pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(mut n: u32) -> u32 {
        let mut result = 0;
        let mut base = 1;
        let mut prev = 9;

        while n != 0 {
            let digit = n % 10;

            n /= 10;

            if digit > prev {
                result = base * digit - 1;
                prev = digit - 1;
            } else {
                result += digit * base;
                prev = digit;
            }

            base *= 10;
        }

        result
    }

    pub fn monotone_increasing_digits(n: i32) -> i32 {
        Self::helper(n as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn monotone_increasing_digits(n: i32) -> i32 {
        Self::monotone_increasing_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

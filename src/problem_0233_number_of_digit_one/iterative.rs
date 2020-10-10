pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut n = i64::from(n);
        let mut result = 0;
        let mut ten_to_the_power = 1;
        let mut base = 0;
        let mut processed = 0;

        while n > 0 {
            let digit = n % 10;

            match digit {
                0 => {}
                1 => result += base + processed + 1,
                _ => result += base * digit + ten_to_the_power,
            }

            processed += ten_to_the_power * digit;
            base = ten_to_the_power + base * 10;
            ten_to_the_power *= 10;
            n /= 10;
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn count_digit_one(n: i32) -> i32 {
        Self::count_digit_one(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

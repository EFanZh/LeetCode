pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let n = i64::from(n);
        let mut result = 0;
        let mut ten_to_the_power = 1;

        while ten_to_the_power <= n {
            result += (ten_to_the_power / 10) * (n / ten_to_the_power);

            let remainder = n % (ten_to_the_power * 10);

            if remainder >= ten_to_the_power {
                result += (remainder - ten_to_the_power + 1).min(ten_to_the_power);
            }

            ten_to_the_power *= 10;
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

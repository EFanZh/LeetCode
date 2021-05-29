pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let n = n as u64;
        let mut total_digits = 1;
        let mut number_length = 1;
        let mut number_count = 9;
        let mut base = 1;

        loop {
            let next_total_digits = total_digits + number_length * number_count;

            if next_total_digits > n {
                break;
            }

            total_digits = next_total_digits;
            number_length += 1;
            number_count *= 10;
            base *= 10;
        }

        let remaining_digits = n - total_digits;
        let mut number = base + remaining_digits / number_length;
        let digit_index = remaining_digits % number_length;

        for _ in 1..number_length - digit_index {
            number /= 10;
        }

        (number % 10) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_nth_digit(n: i32) -> i32 {
        Self::find_nth_digit(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

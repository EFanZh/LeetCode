pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut counts = [0_u8; 10];
        let mut n = n.cast_unsigned();

        while n != 0 {
            counts[(n % 10) as usize] += 1;
            n /= 10;
        }

        let mut min_digit = 0;
        let mut min_count = u8::MAX;

        (0..).zip(&counts).for_each(|(digit, &count)| {
            if count != 0 && count < min_count {
                min_digit = digit;
                min_count = count;
            }
        });

        min_digit
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_least_frequent_digit(n: i32) -> i32 {
        Self::get_least_frequent_digit(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

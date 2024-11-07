pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_numbers(num: i32, k: i32) -> i32 {
        let num = num as u16;
        let k = k as u16;

        if num == 0 {
            return 0;
        }

        let num_unit_digit = num % 10;
        let mut x = k;
        let mut count = 1;

        while x <= num {
            let unit_digit = x % 10;

            if unit_digit == num_unit_digit {
                return count;
            }

            if unit_digit == 0 {
                return -1;
            }

            x += k;
            count += 1;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_numbers(num: i32, k: i32) -> i32 {
        Self::minimum_numbers(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

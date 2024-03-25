pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sum_of_digits(mut x: u32) -> u32 {
        let mut result = 0;

        while x != 0 {
            result += x % 10;
            x /= 10;
        }

        result
    }

    pub fn count_even(num: i32) -> i32 {
        let num = num as u32;
        let base_count = num / 10 * 5;
        let extra_count = (num % 10 + (2 - ((num ^ Self::sum_of_digits(num)) & 1))) / 2;

        (base_count + extra_count - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_even(num: i32) -> i32 {
        Self::count_even(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

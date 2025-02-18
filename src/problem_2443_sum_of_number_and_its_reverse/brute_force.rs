pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn reverse(mut num: u32) -> u32 {
        let mut reversed = 0;

        while num != 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        reversed
    }

    pub fn sum_of_number_and_reverse(num: i32) -> bool {
        let num = num as u32;

        (0..=num).rev().any(|x| x + Self::reverse(x) == num)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_of_number_and_reverse(num: i32) -> bool {
        Self::sum_of_number_and_reverse(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

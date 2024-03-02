pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut num = num as u16;
        let a = (num % 10) as u8;

        num /= 10;

        let b = (num % 10) as u8;

        num /= 10;

        let c = (num % 10) as u8;
        let d = (num / 10) as u8;

        let mut digits = [a, b, c, d];

        digits.sort_unstable();

        let [a, b, c, d] = digits;

        i32::from((a + b) * 10 + c + d)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_sum(num: i32) -> i32 {
        Self::minimum_sum(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

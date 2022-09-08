pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut num = num as u32;
        let mut buffer = [0_u8; 10];
        let mut i = 9;

        loop {
            buffer[i] = (num % 10) as _;
            num /= 10;

            if num == 0 {
                break;
            }

            i -= 1;
        }

        let digits = &mut buffer[i..];

        if let Some(digit) = digits.iter_mut().find(|d| **d == 6) {
            *digit = 9;
        }

        let mut result = 0;

        for d in digits {
            result = result * 10 + i32::from(*d);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum69_number(num: i32) -> i32 {
        Self::maximum69_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

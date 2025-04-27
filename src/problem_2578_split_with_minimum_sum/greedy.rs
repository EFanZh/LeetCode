pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut num = num as u32;
        let mut digits = [0_u8; 10];
        let mut i = 0;

        while num != 0 {
            digits[i] = (num % 10) as _;
            num /= 10;
            i += 1;
        }

        let digits = &mut digits[..i];

        digits.sort_unstable_by(|lhs, rhs| rhs.cmp(lhs));

        let mut base = 1;
        let mut iter = digits.chunks_exact(2);
        let mut result = 0;

        for chunk in &mut iter {
            let [x, y] = chunk.try_into().ok().unwrap();

            result += base * u32::from(x + y);
            base *= 10;
        }

        if let [extra] = *iter.remainder() {
            result += base * u32::from(extra);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_num(num: i32) -> i32 {
        Self::split_num(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

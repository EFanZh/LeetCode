pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = n.trailing_zeros();

        loop {
            n >>= n.trailing_zeros();

            if n < 4 {
                result += n - 1;

                break;
            }

            if n & 3 == 1 {
                n -= 1;
            } else {
                n += 1;
            }

            result += 1 + n.trailing_zeros();
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn integer_replacement(n: i32) -> i32 {
        Self::integer_replacement(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 8 {
            let half = n / 2;

            half * (n - half)
        } else {
            let n = n as u32;

            match n % 3 {
                0 => 3_i32.pow(n / 3),
                1 => 3_i32.pow(n / 3 - 1) * 4,
                _ => 3_i32.pow(n / 3) * 2,
            }
        }
    }
}

impl super::Solution for Solution {
    fn integer_break(n: i32) -> i32 {
        Self::integer_break(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

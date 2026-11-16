pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n.cast_unsigned();

        (if n < 8 {
            let half = n / 2;

            half * (n - half)
        } else {
            match n % 3 {
                0 => u32::pow(3, n / 3),
                1 => u32::pow(3, n / 3 - 1) * 4,
                _ => u32::pow(3, n / 3) * 2,
            }
        })
        .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

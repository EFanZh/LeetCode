pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let mut base = 2_u64;
        let mut exponent = n;
        let mut result = 1;

        loop {
            if exponent & 1 != 0 {
                result = (result * base) % MODULUS;
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = (base * base) % MODULUS;
        }

        ((result + MODULUS - 2) % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn monkey_move(n: i32) -> i32 {
        Self::monkey_move(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

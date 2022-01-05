pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u64 = 1_000_000_007;

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        loop {
            if y == 0 {
                return x;
            }

            x %= y;

            if x == 0 {
                return y;
            }

            y %= x;
        }
    }

    fn mul_sub_div(lhs: u64, rhs: u64, subtract: u64, denominator: u64) -> u64 {
        if let Some(numerator) = lhs.checked_mul(rhs) {
            (numerator - subtract) / denominator
        } else {
            ((u128::from(lhs) * u128::from(rhs) - u128::from(subtract)) / u128::from(denominator)) as _
        }
    }

    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let (n, a, b) = (n as u64, a as u64, b as u64);
        let gcd = u64::from(Self::gcd(a as _, b as _));
        let lcm = a * b / gcd;
        let top = a * b * lcm;
        let base = a * lcm + b * lcm - a * b;
        let mut low = Self::mul_sub_div(top, n - 1, 0, base) + 1;
        let mut high = Self::mul_sub_div(top, n + 2, 1, base);

        while low < high {
            let middle = (low + high) / 2;

            if middle / a + middle / b - middle / lcm < n {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        (low % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        Self::nth_magical_number(n, a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

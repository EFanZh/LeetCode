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

    fn div_ceil(lhs: u64, rhs: u64) -> u64 {
        (lhs + (rhs - 1)) / rhs
    }

    fn search_in_cycle(n: u32, a: u32, b: u32) -> u32 {
        let (n_u64, a_u64, b_u64) = (u64::from(n), u64::from(a), u64::from(b));
        let mut low = Self::div_ceil(a_u64 * b_u64 * n_u64, a_u64 + b_u64) as u32;
        let mut high = ((a_u64 * b_u64 * (n_u64 + 2) - 1) / (a_u64 + b_u64)) as u32;

        while low < high {
            let middle = (low + high) / 2;

            if middle / a + middle / b < n {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        low
    }

    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let (n, a, b) = (n as u32, a as u32, b as u32);
        let lcm = a * b / Self::gcd(a, b);
        let cycle_length = lcm / a + lcm / b - 1;
        let cycles = n / cycle_length;
        let remaining = Self::search_in_cycle(n % cycle_length, a, b);

        (((u64::from(lcm) * u64::from(cycles) % MODULUS) + u64::from(remaining)) % MODULUS) as _
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

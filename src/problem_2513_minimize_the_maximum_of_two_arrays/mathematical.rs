pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    /// Returns smallest `x` such that `x - x / divisor >= count`.
    fn needed(count: u32, divisor: u32) -> u32 {
        count + (count - 1) / (divisor - 1)
    }

    pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        let divisor1 = divisor1.cast_unsigned();
        let divisor2 = divisor2.cast_unsigned();
        let unique_cnt1 = unique_cnt1.cast_unsigned();
        let unique_cnt2 = unique_cnt2.cast_unsigned();
        let total_count = unique_cnt1 + unique_cnt2;
        let lcm = (divisor1 / Self::gcd(divisor1, divisor2)).checked_mul(divisor2);

        Self::needed(unique_cnt1, divisor1)
            .max(Self::needed(unique_cnt2, divisor2))
            .max(lcm.map_or(total_count, |lcm| Self::needed(total_count, lcm)))
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        Self::minimize_set(divisor1, divisor2, unique_cnt1, unique_cnt2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

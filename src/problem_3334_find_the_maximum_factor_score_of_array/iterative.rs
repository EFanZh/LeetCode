pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    fn lcm(mut x: u64, mut y: u64) -> u64 {
        let product = x * y;

        while y != 0 {
            (x, y) = (y, x % y);
        }

        product / x
    }

    pub fn max_score(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut gcd = 0;
        let mut lcm = 1;

        let right = nums
            .iter()
            .rev()
            .map(|&num| {
                let result = (gcd, lcm);

                gcd = Self::gcd(num, gcd);
                lcm = Self::lcm(u64::from(num), lcm);

                result
            })
            .collect::<Vec<_>>();

        let mut result = u64::from(gcd) * lcm;
        let mut gcd = 0;
        let mut lcm = 1;

        for (&num, &(right_gcd, right_lcm)) in nums.iter().zip(right.iter().rev()) {
            result = result.max(u64::from(Self::gcd(right_gcd, gcd)) * Self::lcm(right_lcm, lcm));

            gcd = Self::gcd(num, gcd);
            lcm = Self::lcm(u64::from(num), lcm);
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(nums: Vec<i32>) -> i64 {
        Self::max_score(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

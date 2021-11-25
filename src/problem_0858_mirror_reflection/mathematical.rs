pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut lhs: u32, mut rhs: u32) -> u32 {
        loop {
            let remainder_1 = lhs % rhs;

            if remainder_1 == 0 {
                return rhs;
            }

            let remainder_2 = rhs % remainder_1;

            if remainder_2 == 0 {
                return remainder_1;
            }

            lhs = remainder_1;
            rhs = remainder_2;
        }
    }

    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let (p, q) = (p as _, q as _);
        let gcd = Self::gcd(p, q);
        let (p, q) = (p / gcd, q / gcd);

        if p % 2 == 0 {
            2
        } else if q % 2 == 0 {
            0
        } else {
            1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mirror_reflection(p: i32, q: i32) -> i32 {
        Self::mirror_reflection(p, q)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut x: u16, mut y: u16) -> u16 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    pub fn common_factors(a: i32, b: i32) -> i32 {
        let a = a as u16;
        let b = b as u16;
        let gcd = Self::gcd(a, b);
        let middle = f32::from(gcd).sqrt() as u16;
        let mut result = 0;

        for i in 1..middle {
            result += u8::from(gcd % i == 0);
        }

        result = result * 2
            + if middle * middle == gcd {
                1
            } else {
                u8::from(gcd % middle == 0) * 2
            };

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn common_factors(a: i32, b: i32) -> i32 {
        Self::common_factors(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

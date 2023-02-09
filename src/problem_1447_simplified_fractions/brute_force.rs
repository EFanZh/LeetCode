pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn gcd(mut x: NonZeroU32, mut y: u32) -> NonZeroU32 {
        while let Some(remainder) = NonZeroU32::new(y % x) {
            y = x.get();
            x = remainder;
        }

        x
    }

    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let n = n as u32;
        let mut result = Vec::new();

        for y in 2..=n {
            let mut x_candidate = y - 1;

            while let Some(x) = NonZeroU32::new(x_candidate) {
                if Self::gcd(x, y).get() == 1 {
                    result.push(format!("{x}/{y}"));
                }

                x_candidate -= 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn simplified_fractions(n: i32) -> Vec<String> {
        Self::simplified_fractions(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

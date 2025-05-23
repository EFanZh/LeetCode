pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut guess = f64::from_bits(f64::from(x).to_bits() / 2 + 0x1FF8_0000_0000_0000) as u32;

        if guess * guess > x {
            guess = u32::midpoint(guess, x / guess);

            if guess * guess > x {
                guess = u32::midpoint(guess, x / guess);

                if guess * guess > x {
                    guess = u32::midpoint(guess, x / guess);
                }
            }
        }

        guess as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

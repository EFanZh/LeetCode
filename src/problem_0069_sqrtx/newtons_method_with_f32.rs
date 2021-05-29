pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::cast_precision_loss)]
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut guess = f32::from_bits((x as f32).to_bits() / 2 + 0x1FC0_0000) as u32;

        if guess * guess > x {
            guess = (guess + x / guess) / 2;

            if guess * guess > x {
                guess = (guess + x / guess) / 2;

                if guess * guess > x {
                    guess = (guess + x / guess) / 2;
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

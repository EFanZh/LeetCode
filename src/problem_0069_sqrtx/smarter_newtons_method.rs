pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut guess = (1 << ((33 - x.leading_zeros()) / 2)) - 1;

        if guess * guess > x {
            guess = u32::midpoint(guess, x / guess);

            if guess * guess > x {
                guess = u32::midpoint(guess, x / guess);

                if guess * guess > x {
                    guess = u32::midpoint(guess, x / guess);

                    if guess * guess > x {
                        guess = u32::midpoint(guess, x / guess);

                        if guess * guess > x {
                            guess = u32::midpoint(guess, x / guess);
                        }
                    }
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

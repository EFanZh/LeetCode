pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        let mut n = n.cast_unsigned();

        if n > 5 {
            loop {
                let mut x = n;
                let mut sum = 0;
                let mut factor = 2;
                let mut factor_squared = factor * factor;

                loop {
                    if x.is_multiple_of(factor) {
                        x /= factor;
                        sum += factor;
                    } else {
                        factor += 1;
                        factor_squared = factor * factor;
                    }

                    if factor_squared >= x {
                        break;
                    }
                }

                sum += if factor_squared == x { factor * 2 } else { x };

                if sum == n {
                    break;
                }

                n = sum;
            }
        }

        n.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_value(n: i32) -> i32 {
        Self::smallest_value(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = 0;

        while n != 0 {
            n >>= n.trailing_zeros();

            loop {
                // Invariant: `n.trailing_ones() != 0`.

                result += 1;

                let trailing_ones = n.trailing_ones();
                let has_one_one = trailing_ones < 2;

                n += u32::from(!has_one_one);
                n >>= trailing_ones;

                if has_one_one {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(n: i32) -> i32 {
        Self::min_operations(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

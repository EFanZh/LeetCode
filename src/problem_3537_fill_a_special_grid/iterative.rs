pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
        let size = 1_u16 << n.cast_unsigned();
        let initial_probe = size >> 1;

        (0..size)
            .map(|y| {
                (0..size)
                    .map(|x| {
                        let mut probe = initial_probe;
                        let mut result = 0;

                        while probe != 0 {
                            result <<= 2;

                            result |= match (y & probe, x & probe) {
                                (0, 0) => 3,
                                (0, _) => 0,
                                (_, 0) => 2,
                                (_, _) => 1,
                            };

                            probe >>= 1;
                        }

                        result
                    })
                    .collect()
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn special_grid(n: i32) -> Vec<Vec<i32>> {
        Self::special_grid(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

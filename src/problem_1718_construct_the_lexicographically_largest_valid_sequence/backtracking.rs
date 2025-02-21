pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[expect(clippy::unnecessary_map_or, reason = "compatibility")]
    fn helper(buffer: &mut [i32], available: u32) -> bool {
        buffer.split_first_mut().map_or(true, |(first, rest)| {
            if *first == 0 {
                let mut bits = available;

                while bits != 0 {
                    let candidate = 32 - bits.trailing_zeros();
                    let bit = 1 << bits.trailing_zeros();

                    if candidate == 1 {
                        *first = candidate as _;

                        if Self::helper(rest, available ^ bit) {
                            return true;
                        }

                        *first = 0;
                    } else {
                        let other_index = candidate as usize - 1;

                        if rest.get(other_index).copied() == Some(0) {
                            *first = candidate as _;
                            rest[other_index] = candidate as _;

                            if Self::helper(rest, available ^ bit) {
                                return true;
                            }

                            *first = 0;
                            rest[other_index] = 0;
                        }
                    }

                    bits ^= bit;
                }

                false
            } else {
                Self::helper(rest, available)
            }
        })
    }

    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as u32;
        let mut result = vec![0; n as usize * 2 - 1];

        Self::helper(&mut result, u32::MAX << (32 - n));

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        Self::construct_distanced_sequence(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

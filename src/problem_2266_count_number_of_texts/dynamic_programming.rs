pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(target: &mut u32, value: u32) {
        *target += value;
        *target = target.checked_sub(1_000_000_007).unwrap_or(*target);
    }

    pub fn count_texts(pressed_keys: String) -> i32 {
        let mut cache = [1_u32; 4];
        let mut head = 0;
        let mut prev = 0;
        let mut length_minus_1 = u8::MAX;

        for c in pressed_keys.bytes() {
            if c == prev {
                length_minus_1 = length_minus_1.wrapping_add(1).min(3);
            } else {
                prev = c;
                length_minus_1 = 0;
            }

            let mut count = cache[head & 3];

            'block_0: {
                'block_1: {
                    match length_minus_1 {
                        0 => break 'block_0,
                        1 => break 'block_1,
                        2 => {}
                        _ => {
                            if matches!(prev, b'7' | b'9') {
                                Self::add(&mut count, cache[head.wrapping_add(3) & 3]);
                            }
                        }
                    }

                    Self::add(&mut count, cache[head.wrapping_add(2) & 3]);
                }

                Self::add(&mut count, cache[head.wrapping_add(1) & 3]);
            }

            head = head.wrapping_sub(1);
            cache[head & 3] = count;
        }

        cache[head & 3] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_texts(pressed_keys: String) -> i32 {
        Self::count_texts(pressed_keys)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

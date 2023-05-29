pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn iter_bits(mut bits: u32, mut f: impl FnMut(usize, u32)) {
        while bits != 0 {
            let i = bits.trailing_zeros();
            let bit = 1 << i;

            f(i as usize, bit);

            bits ^= bit;
        }
    }

    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let target = !target as u32;
        let mut start = 0;
        let mut bit_counts = [0_u32; 32];
        let mut bits = 0_u32;
        let mut min_diff = u32::MAX;

        for (i, &new) in arr.iter().enumerate() {
            Self::iter_bits(!new as _, |i, bit| {
                bit_counts[i] += 1;
                bits |= bit;
            });

            match bits.cmp(&target) {
                Ordering::Less => min_diff = target - bits,
                Ordering::Equal => return 0,
                Ordering::Greater => loop {
                    let old = !arr[start] as u32;
                    let mut next_bits = bits;

                    Self::iter_bits(old, |i, bit| {
                        if bit_counts[i] == 1 {
                            next_bits ^= bit;
                        }
                    });

                    match next_bits.cmp(&target) {
                        Ordering::Less => {
                            min_diff = min_diff.min(bits - target);

                            if start < i {
                                min_diff = min_diff.min(target - next_bits);
                            }

                            break;
                        }
                        Ordering::Equal => return 0,
                        Ordering::Greater => {
                            start += 1;

                            Self::iter_bits(old, |i, _| bit_counts[i] -= 1);

                            bits = next_bits;
                        }
                    }
                },
            }
        }

        min_diff as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        Self::closest_to_target(arr, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

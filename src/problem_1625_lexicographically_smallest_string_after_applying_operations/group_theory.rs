pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

// See <https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/discuss/899708/Proof-Group-Theory-or-Java-14ms-100-or-Optimal-O(N2)-time-O(N)-space>.

impl Solution {
    fn gcd(mut lhs: NonZeroU8, mut rhs: u8) -> NonZeroU8 {
        while let Some(non_zero_rhs) = NonZeroU8::new(rhs) {
            let new_rhs = lhs.get() % non_zero_rhs;

            lhs = non_zero_rhs;
            rhs = new_rhs;
        }

        lhs
    }

    fn add_from(s: &mut [u8], mut i: usize, k: u8) {
        while let Some(c) = s.get_mut(i) {
            *c += k;

            if *c > b'9' {
                *c -= 10;
            }

            i += 2;
        }
    }

    fn add_even(s: &mut [u8], k: u8) {
        Self::add_from(s, 0, k);
    }

    fn add_odd(s: &mut [u8], k: u8) {
        Self::add_from(s, 1, k);
    }

    fn rotate(s: &mut [u8], k: u8) {
        s.rotate_right(usize::from(k));
    }

    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut s = s.into_bytes();
        let s = s.as_mut_slice();
        let n = s.len() as u8;
        let a = a as u8;
        let b = b as u8;
        let a_step = Self::gcd(NonZeroU8::new(10).unwrap(), a);
        let a_cycle = 10 / a_step;
        let b_step = Self::gcd(NonZeroU8::new(n).unwrap(), b);
        let b_cycle = n / b_step;
        let mut best = s.to_vec().into_boxed_slice();

        if b & 1 == 0 {
            for _ in 0..a_cycle {
                Self::add_odd(s, a_step.get());

                for _ in 0..b_cycle {
                    Self::rotate(s, b_step.get());

                    if *s < *best {
                        best.copy_from_slice(s);
                    }
                }
            }
        } else {
            for _ in 0..a_cycle {
                Self::add_odd(s, a_step.get());

                for _ in 0..a_cycle {
                    Self::add_even(s, a_step.get());

                    for _ in 0..b_cycle {
                        Self::rotate(s, b_step.get());

                        if *s < *best {
                            best.copy_from_slice(s);
                        }
                    }
                }
            }
        }

        String::from_utf8(Vec::from(best)).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        Self::find_lex_smallest_string(s, a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

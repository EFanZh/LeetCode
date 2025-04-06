pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::num::NonZeroU32;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut s = s.into_bytes();
        let repeat_limit = NonZeroU32::new(repeat_limit as _).unwrap();
        let mut counts = [0_u32; 26];

        for &c in &s {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        s.clear();

        let mut i = 25;

        'outer: while let Some(count) = counts.get(i).copied() {
            if count == 0 {
                i = i.wrapping_sub(1);

                continue;
            }

            let mut cycles = count / repeat_limit;
            let mut extra = count % repeat_limit;

            if extra == 0 {
                cycles -= 1;
                extra = repeat_limit.get();
            }

            let mut j = i.wrapping_sub(1);

            for _ in 0..cycles {
                s.extend(iter::repeat_n(b'a' + i as u8, repeat_limit.get() as _));

                loop {
                    if let Some(count_2) = counts.get_mut(j) {
                        if *count_2 == 0 {
                            j = j.wrapping_sub(1);
                        } else {
                            *count_2 -= 1;

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }

                s.push(b'a' + j as u8);
            }

            s.extend(iter::repeat_n(b'a' + i as u8, extra as _));

            i = j;
        }

        String::from_utf8(s).ok().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        Self::repeat_limited_string(s, repeat_limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

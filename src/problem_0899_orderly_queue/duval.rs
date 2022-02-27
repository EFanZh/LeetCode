pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn duval(s: &mut [u8]) {
        // See <https://cp-algorithms.com/string/lyndon_factorization.html>.

        let length = s.len();
        let mut i = 0;
        let mut split = 0;
        let char_at = |raw: usize| s.get(raw.checked_sub(length).unwrap_or(raw));

        while i < length {
            let mut k = i;
            let mut j = i + 1;

            while let Some(right) = char_at(j) {
                match char_at(k).unwrap().cmp(right) {
                    Ordering::Less => {
                        // `s[i..=j]` is simple.

                        k = i;
                        j += 1;
                    }
                    Ordering::Equal => {
                        // `s[i..=j]` is pre-simple.

                        k += 1;
                        j += 1;
                    }
                    Ordering::Greater => {
                        // Split `s[i..=j]`.

                        break;
                    }
                }
            }

            split = i;

            while i <= k {
                i += j - k;
            }
        }

        s.rotate_left(split);
    }

    fn counting_sort(mut s: &mut [u8]) {
        let mut counts = [0_u16; 26];

        for &c in &*s {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        for (c, &count) in (b'a'..).zip(&counts) {
            let (left, right) = s.split_at_mut(usize::from(count));

            left.fill(c);

            s = right;
        }
    }

    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut s = s.into_bytes();

        if k == 1 {
            Self::duval(&mut s);
        } else {
            Self::counting_sort(&mut s);
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn orderly_queue(s: String, k: i32) -> String {
        Self::orderly_queue(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::iter;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut s = s.into_bytes();

        let mut counts = [
            (b'a', 0),
            (b'b', 0),
            (b'c', 0),
            (b'd', 0),
            (b'e', 0),
            (b'f', 0),
            (b'g', 0),
            (b'h', 0),
            (b'i', 0),
            (b'j', 0),
            (b'k', 0),
            (b'l', 0),
            (b'm', 0),
            (b'n', 0),
            (b'o', 0),
            (b'p', 0),
            (b'q', 0),
            (b'r', 0),
            (b's', 0),
            (b't', 0),
            (b'u', 0),
            (b'v', 0),
            (b'w', 0),
            (b'x', 0),
            (b'y', 0),
            (b'z', 0),
        ];

        for &c in &s {
            let count = &mut counts[usize::from(c) - usize::from(b'a')].1;

            *count += 1;
        }

        counts.sort_unstable_by_key(|&(letter, count)| (Reverse(count), letter));

        let buckets = counts[0].1;
        let n = s.len();

        if buckets <= (n + 1) / 2 {
            let mut iter = counts.iter().flat_map(|&(c, count)| iter::repeat(c).take(count));

            for i in (0..n).step_by(2) {
                s[i] = iter.next().unwrap();
            }

            for i in (1..n).step_by(2) {
                s[i] = iter.next().unwrap();
            }
        } else {
            s.clear();
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reorganize_string(s: String) -> String {
        Self::reorganize_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

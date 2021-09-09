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

        if buckets <= (s.len() + 1) / 2 {
            let mut iter = counts.iter().flat_map(|&(c, count)| iter::repeat(c).take(count));
            let n = s.len();
            let small_bucket_size = n / buckets;
            let large_bucket_count = n % buckets;
            let large_bucket_size = small_bucket_size + 1;
            let small_bucket_count = buckets - large_bucket_count;
            let mut index = 0;

            for _ in 0..small_bucket_size {
                for _ in 0..large_bucket_count {
                    s[index] = iter.next().unwrap();
                    index += large_bucket_size;
                }

                for _ in 0..small_bucket_count {
                    s[index] = iter.next().unwrap();
                    index += small_bucket_size;
                }

                index -= n - 1;
            }

            for _ in 0..large_bucket_count {
                s[index] = iter.next().unwrap();
                index += large_bucket_size;
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

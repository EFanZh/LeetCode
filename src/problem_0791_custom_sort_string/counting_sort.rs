pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut orders = [usize::MAX; 26];

        for (i, c) in order.bytes().enumerate() {
            orders[usize::from(c) - usize::from(b'a')] = i;
        }

        let mut counts = [0_usize; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut s = s.into_bytes();

        s.clear();

        for c in order.bytes() {
            s.extend(iter::repeat(c).take(counts[usize::from(c) - usize::from(b'a')]));
        }

        for (c, (&count, &order)) in (b'a'..).zip(counts.iter().zip(&orders)) {
            if order == usize::MAX {
                s.extend(iter::repeat(c).take(count));
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn custom_sort_string(order: String, s: String) -> String {
        Self::custom_sort_string(order, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

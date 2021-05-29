pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        const START: u8 = b' ';
        const END: u8 = b'z';

        let mut s = s.into_bytes();
        let mut counts = [(0, 0); (END - START + 1) as _];

        for ((target, _), c) in counts.iter_mut().zip(START..) {
            *target = c;
        }

        for c in &s {
            counts[usize::from(c - START)].1 += 1;
        }

        counts.sort_unstable_by_key(|(_, count)| *count);

        s.clear();

        for &(c, count) in counts.iter().rev() {
            s.resize(s.len() + count, c);
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn frequency_sort(s: String) -> String {
        Self::frequency_sort(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn counting_sort(s: &mut [u8]) {
        let mut counts = [0_u32; 26];

        for &c in &*s {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut i = 0;

        for (c, count) in (b'a'..).zip(counts) {
            let count = count as usize;
            let j = i + count;

            s[i..j].fill(c);
            i = j;
        }
    }

    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();

        Self::counting_sort(&mut s1);
        Self::counting_sort(&mut s2);

        let mut iter = s1.iter().zip(&s2).map(|(lhs, rhs)| lhs.cmp(rhs));

        while let Some(ordering) = iter.next() {
            match ordering {
                Ordering::Less => return iter.all(|ordering| ordering != Ordering::Greater),
                Ordering::Equal => {}
                Ordering::Greater => return iter.all(|ordering| ordering != Ordering::Less),
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_if_can_break(s1: String, s2: String) -> bool {
        Self::check_if_can_break(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

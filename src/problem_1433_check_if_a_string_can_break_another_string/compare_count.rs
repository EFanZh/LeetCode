pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut counts = [0; 26];

        for c in s1.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        for c in s2.bytes() {
            counts[usize::from(c) - usize::from(b'a')] -= 1;
        }

        let mut iter = counts.iter().scan(0, |sum, &x| {
            *sum += x;

            Some(*sum)
        });

        while let Some(sum) = iter.next() {
            match sum.cmp(&0) {
                Ordering::Less => return iter.all(|sum| sum <= 0),
                Ordering::Equal => {}
                Ordering::Greater => return iter.all(|sum| sum >= 0),
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

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut iter = s.bytes();

        let mut largest = loop {
            if let Some(c) = iter.next() {
                if c.is_ascii_digit() {
                    break c;
                }
            } else {
                return -1;
            }
        };

        let mut second_largest = loop {
            if let Some(c) = iter.next() {
                if c.is_ascii_digit() {
                    break match c.cmp(&largest) {
                        Ordering::Less => c,
                        Ordering::Equal => continue,
                        Ordering::Greater => mem::replace(&mut largest, c),
                    };
                }
            } else {
                return -1;
            }
        };

        for c in iter {
            if c.is_ascii_digit() && c > second_largest {
                match c.cmp(&largest) {
                    Ordering::Less => second_largest = c,
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        second_largest = largest;
                        largest = c;
                    }
                }
            }
        }

        i32::from(second_largest - b'0')
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn second_highest(s: String) -> i32 {
        Self::second_highest(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

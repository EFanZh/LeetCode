pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn gcd(mut lhs: u16, mut rhs: u16) -> u16 {
        loop {
            if rhs == 0 {
                return lhs;
            }

            lhs %= rhs;

            if lhs == 0 {
                return rhs;
            }

            rhs %= lhs;
        }
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counts = HashMap::new();

        for &value in &deck {
            counts.entry(value).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut iter = counts.values().copied();
        let mut x = iter.next().unwrap();

        for value in iter {
            x = Self::gcd(value, x);
        }

        x > 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_groups_size_x(deck: Vec<i32>) -> bool {
        Self::has_groups_size_x(deck)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

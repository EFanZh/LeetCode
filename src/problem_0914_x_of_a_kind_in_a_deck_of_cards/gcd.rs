pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::num::NonZeroU16;

impl Solution {
    fn gcd(mut lhs: u16, mut rhs: NonZeroU16) -> NonZeroU16 {
        while let Some(new_rhs) = NonZeroU16::new(lhs % rhs) {
            lhs = rhs.get();
            rhs = new_rhs;
        }

        rhs
    }

    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut counts = HashMap::new();

        for &value in &deck {
            counts.entry(value).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut iter = counts.values().copied();
        let mut gcd = iter.next().and_then(NonZeroU16::new).unwrap();

        for value in iter {
            gcd = Self::gcd(value, gcd);
        }

        gcd.get() != 1
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

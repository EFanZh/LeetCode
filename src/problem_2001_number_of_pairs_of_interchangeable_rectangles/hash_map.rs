pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU32;

struct Ratio {
    numerator: u32,
    denominator: u32,
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        u64::from(self.numerator) * u64::from(other.denominator)
            == u64::from(other.numerator) * u64::from(self.denominator)
    }
}

impl Eq for Ratio {}

impl Hash for Ratio {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self {
            mut numerator,
            mut denominator,
        } = *self;

        while let Some(non_zero_denominator) = NonZeroU32::new(denominator) {
            let quotient = numerator / non_zero_denominator;
            let remainder = numerator % non_zero_denominator;

            quotient.hash(state);

            numerator = denominator;
            denominator = remainder;
        }
    }
}

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut counts = HashMap::<_, u32>::new();
        let mut result = 0_u64;

        for rectangle in rectangles {
            let [width, height]: [_; 2] = rectangle.try_into().ok().unwrap();

            match counts.entry(Ratio {
                numerator: width as _,
                denominator: height as _,
            }) {
                Entry::Occupied(entry) => {
                    let count = entry.into_mut();

                    result += u64::from(*count);

                    *count += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        Self::interchangeable_rectangles(rectangles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

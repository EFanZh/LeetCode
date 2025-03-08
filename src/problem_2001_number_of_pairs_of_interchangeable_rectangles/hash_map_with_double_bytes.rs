pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut counts = HashMap::<_, u32>::new();
        let mut result = 0_u64;

        for rectangle in rectangles {
            let [width, height] = rectangle.try_into().ok().unwrap();

            match counts.entry((f64::from(width) / f64::from(height)).to_ne_bytes()) {
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

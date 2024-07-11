pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_length = 0;
        let mut count = 0;

        for rectangle in rectangles {
            let [length, width]: [_; 2] = rectangle.try_into().ok().unwrap();
            let min_length = length.min(width);

            match min_length.cmp(&max_length) {
                Ordering::Less => {}
                Ordering::Equal => count += 1,
                Ordering::Greater => {
                    max_length = min_length;
                    count = 1;
                }
            }
        }

        count
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        Self::count_good_rectangles(rectangles)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

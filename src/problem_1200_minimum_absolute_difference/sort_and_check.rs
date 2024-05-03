pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;

        arr.sort_unstable();

        let mut result = Vec::<Vec<_>>::new();
        let mut length = 0;
        let mut min_diff = i32::MAX;

        let mut iter = arr.into_iter();
        let mut left = iter.next().unwrap();

        for right in iter {
            let diff = right - left;

            match diff.cmp(&min_diff) {
                Ordering::Less => {
                    min_diff = diff;

                    if let Some(slot) = result.first_mut() {
                        slot.clear();
                        slot.extend([left, right]);
                    } else {
                        result.push(vec![left, right]);
                    }

                    length = 1;
                }
                Ordering::Equal => {
                    if let Some(slot) = result.get_mut(length) {
                        slot.clear();
                        slot.extend([left, right]);
                    } else {
                        result.push(vec![left, right]);
                    }

                    length += 1;
                }
                Ordering::Greater => {}
            }

            left = right;
        }

        result.truncate(length);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        Self::minimum_abs_difference(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

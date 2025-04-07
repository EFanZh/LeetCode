pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut iter_1 = nums1.iter().copied();
        let iter_2 = nums2.iter().copied();

        iter_1.next().map_or(-1, |mut left| {
            'outer: for right in iter_2 {
                loop {
                    match left.cmp(&right) {
                        Ordering::Less => {}
                        Ordering::Equal => return left,
                        Ordering::Greater => break,
                    }

                    if let Some(next_left) = iter_1.next() {
                        left = next_left;
                    } else {
                        break 'outer;
                    }
                }
            }

            -1
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::get_common(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

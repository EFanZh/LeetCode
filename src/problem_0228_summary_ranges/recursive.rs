pub struct Solution;

use std::vec::IntoIter;

impl Solution {
    fn summary_ranges_helper_multiple(iter: &mut IntoIter<i32>, first: i32, last: i32, result: &mut Vec<String>) {
        if let Some(num) = iter.next() {
            if num == last + 1 {
                Self::summary_ranges_helper_multiple(iter, first, num, result);
            } else {
                result.push(format!("{}->{}", first, last));

                Self::summary_ranges_helper_single(iter, num, result);
            }
        } else {
            result.push(format!("{}->{}", first, last));
        }
    }

    fn summary_ranges_helper_single(iter: &mut IntoIter<i32>, first: i32, result: &mut Vec<String>) {
        if let Some(last) = iter.next() {
            if last == first + 1 {
                Self::summary_ranges_helper_multiple(iter, first, last, result);
            } else {
                result.push(first.to_string());

                Self::summary_ranges_helper_single(iter, last, result);
            }
        } else {
            result.push(first.to_string());
        }
    }

    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut iter = nums.into_iter();

        if let Some(first) = iter.next() {
            Self::summary_ranges_helper_single(&mut iter, first, &mut result);
        }

        result
    }
}

impl super::Solution for Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        Self::summary_ranges(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution {}

use std::cmp::Ordering;
use std::mem;

impl Solution {
    fn partition_by<T, P: FnMut(&T) -> bool>(mut values: &mut [T], mut predicate: P) -> usize {
        let mut result = 0;

        'k: while let Some((first, mut rest)) = values.split_first_mut() {
            if predicate(first) {
                values = rest;
                result += 1;
            } else {
                while let Some((last, body)) = rest.split_last_mut() {
                    if predicate(last) {
                        mem::swap(first, last);
                        values = body;
                        result += 1;

                        continue 'k;
                    } else {
                        rest = body;
                    }
                }

                break;
            }
        }

        result
    }

    fn nth_element<T: Ord + Clone>(mut values: &mut [T], mut n: usize) -> T {
        loop {
            let (last, rest) = values.split_last_mut().unwrap();
            let left_length = Self::partition_by(rest, |x| x < last);

            match n.cmp(&left_length) {
                Ordering::Less => values = &mut values[..left_length],
                Ordering::Equal => return last.clone(),
                Ordering::Greater => {
                    values.swap(left_length, values.len() - 1);
                    values = &mut values[left_length + 1..];
                    n -= left_length + 1;
                }
            }
        }
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - (k as usize);

        Self::nth_element(&mut nums, k)
    }
}

impl super::Solution for Solution {
    fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        Self::find_kth_largest(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

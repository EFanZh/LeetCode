pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    fn inner(arr: &[i32], target: u32) -> usize {
        let mut start = 0;
        let mut length = 0;
        let mut sum = 0;
        let mut windows = VecDeque::<(usize, usize)>::new();
        let mut min_left_length = usize::MAX;
        let mut result = usize::MAX;

        for &num in arr {
            sum += num as u32;
            length += 1;

            loop {
                match sum.cmp(&target) {
                    Ordering::Less => break,
                    Ordering::Equal => {
                        while let Some(&(left_start, left_length)) = windows.front() {
                            if left_start + left_length <= start {
                                windows.pop_front();

                                min_left_length = min_left_length.min(left_length);
                            } else {
                                break;
                            }
                        }

                        if let Some(new_result) = min_left_length.checked_add(length) {
                            result = result.min(new_result);
                        }

                        windows.push_back((start, length));

                        break;
                    }
                    Ordering::Greater => {
                        sum -= arr[start] as u32;
                        start += 1;
                        length -= 1;
                    }
                }
            }
        }

        result
    }

    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        Self::inner(&arr, target as _) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        Self::min_sum_of_lengths(arr, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

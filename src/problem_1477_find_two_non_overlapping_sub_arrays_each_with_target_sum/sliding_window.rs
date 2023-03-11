pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

#[derive(Default)]
struct Window {
    start: usize,
    length: usize,
    sum: u32,
}

impl Window {
    fn slide(&mut self, arr: &[i32], target: u32, num: u32) -> bool {
        self.sum += num;
        self.length += 1;

        loop {
            match self.sum.cmp(&target) {
                Ordering::Less => break false,
                Ordering::Equal => break true,
                Ordering::Greater => {
                    self.sum -= arr[self.start] as u32;
                    self.start += 1;
                    self.length -= 1;
                }
            }
        }
    }
}

impl Solution {
    fn inner(arr: &[i32], target: u32) -> usize {
        let mut left_window = Window::default();
        let mut left_min_length = usize::MAX;
        let mut right_window = Window::default();
        let mut result = usize::MAX;

        for &right_num in arr {
            if right_window.slide(arr, target, right_num as _) {
                for &left_num in &arr[left_window.start + left_window.length..right_window.start] {
                    if left_window.slide(arr, target, left_num as _) {
                        left_min_length = left_min_length.min(left_window.length);
                    }
                }

                if let Some(new_result) = left_min_length.checked_add(right_window.length) {
                    result = result.min(new_result);
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

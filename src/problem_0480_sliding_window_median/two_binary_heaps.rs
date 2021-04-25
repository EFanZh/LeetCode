pub struct Solution;

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

struct Window(BinaryHeap<(i32, Reverse<usize>)>, BinaryHeap<Reverse<(i32, usize)>>);

impl Solution {
    fn build_window(nums: &[i32]) -> Window {
        let mut data = nums
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, Reverse(i)))
            .collect::<Vec<_>>();

        data.sort_unstable();

        let right = data
            .drain(data.len() / 2..)
            .map(|(x, Reverse(i))| Reverse((x, i)))
            .collect::<BinaryHeap<_>>();

        Window(BinaryHeap::from(data), right)
    }

    fn helper(mut window: Window, nums: &[i32], mut get_median: impl FnMut(&Window) -> f64) -> Vec<f64> {
        let k = window.0.len() + window.1.len();
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        result.push(get_median(&window));

        for (i, (&old, &new)) in nums.iter().zip(&nums[k..]).enumerate() {
            println!(
                "{:?} {:?}",
                window
                    .0
                    .iter()
                    .copied()
                    .filter_map(|(x, j)| if j.0 >= i { Some(x) } else { None })
                    .collect::<Vec<_>>(),
                window
                    .1
                    .iter()
                    .copied()
                    .filter_map(|Reverse((x, j))| if j >= i { Some(x) } else { None })
                    .collect::<Vec<_>>(),
            );

            if new >= window.1.peek().unwrap().0 .0 {
                window.1.push(Reverse((new, k + i)));

                if old <= window.1.peek().unwrap().0 .0 {
                    let (x, i) = window.1.pop().unwrap().0;

                    window.0.push((x, Reverse(i)));
                }
            } else {
                window.0.push((new, Reverse(k + i)));

                if old >= window.1.peek().unwrap().0 .0 {
                    let (x, Reverse(i)) = window.0.pop().unwrap();

                    window.1.push(Reverse((x, i)));
                }
            }

            while let Some(top) = window.0.peek_mut() {
                if top.1 .0 <= i {
                    PeekMut::pop(top);
                } else {
                    break;
                }
            }

            while let Some(top) = window.1.peek_mut() {
                if top.0 .1 <= i {
                    PeekMut::pop(top);
                } else {
                    break;
                }
            }

            result.push(get_median(&window));
        }

        result
    }

    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let window = Self::build_window(&nums[..k as _]);

        if k % 2 == 0 {
            Self::helper(window, &nums, |window| {
                (f64::from(window.0.peek().unwrap().0) + f64::from(window.1.peek().unwrap().0 .0)) / 2.0
            })
        } else {
            Self::helper(window, &nums, |window| f64::from(window.1.peek().unwrap().0 .0))
        }
    }
}

impl super::Solution for Solution {
    fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        Self::median_sliding_window(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

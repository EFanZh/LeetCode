pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    #[allow(unused_variables)] // Expected.
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let i_as_u = |x: i32| x as u32;
        let k = k as usize;

        let mut items = speed
            .into_iter()
            .zip(efficiency)
            .map(|(s, e)| (i_as_u(s), i_as_u(e)))
            .collect::<Vec<_>>();

        items.sort_unstable_by_key(|&(_, e)| Reverse(e));

        let (left, right) = items.split_at(k);
        let mut result = 0;
        let mut heap = Vec::new();
        let mut speed_sum = 0;

        for &(speed, efficiency) in left {
            heap.push(Reverse(speed));
            speed_sum += u64::from(speed);
            result = result.max(speed_sum * u64::from(efficiency));
        }

        let mut heap = BinaryHeap::from(heap);

        for &(speed, efficiency) in right {
            let mut top = heap.peek_mut().unwrap();

            if speed > top.0 {
                speed_sum += u64::from(speed - top.0);
                top.0 = speed;

                drop(top);

                result = result.max(speed_sum * u64::from(efficiency));
            }
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        Self::max_performance(n, speed, efficiency, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct TotalF64(f64);

impl PartialEq for TotalF64 {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for TotalF64 {}

impl PartialOrd for TotalF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TotalF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut sum = nums.iter().copied().map(f64::from).sum::<f64>();
        let half = sum / 2.0;

        let mut queue = nums
            .iter()
            .copied()
            .map(|x| TotalF64(f64::from(x as u32)))
            .collect::<BinaryHeap<_>>();

        let mut result = 1;

        loop {
            let mut max = queue.peek_mut().unwrap();
            let max = &mut max.0;

            *max /= 2.0;
            sum -= *max;

            if sum <= half {
                break;
            }

            result += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn halve_array(nums: Vec<i32>) -> i32 {
        Self::halve_array(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::TotalF64;

    #[test]
    fn test_total_f64_partial_eq() {
        assert!(TotalF64(0.0) == TotalF64(0.0));
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

use super::MountainArray;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn partition_point(mut left: u32, mut right: u32, mut f: impl FnMut(u32) -> bool) -> u32 {
        while left < right {
            let middle = u32::midpoint(left, right);

            if f(middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left
    }

    fn binary_search(mut left: u32, mut right: u32, mut f: impl FnMut(u32) -> Ordering) -> Option<u32> {
        while left < right {
            let middle = u32::midpoint(left, right);

            match f(middle) {
                Ordering::Less => left = middle + 1,
                Ordering::Equal => return Some(middle),
                Ordering::Greater => right = middle,
            }
        }

        None
    }

    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let n = mountain_arr.length() as u32;
        let peak = Self::partition_point(1, n - 2, |i| mountain_arr.get(i as _) < mountain_arr.get((i + 1) as _));

        Self::binary_search(0, peak + 1, |i| mountain_arr.get(i as _).cmp(&target))
            .or_else(|| Self::binary_search(peak + 1, n, |i| target.cmp(&mountain_arr.get(i as _))))
            .unwrap_or(u32::MAX) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        Self::find_in_mountain_array(target, mountain_arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

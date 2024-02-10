pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/building-boxes/discuss/1288308/C%2B%2B-O(1)-~0ms-no-loop-math-solution-with-detailed-explanation>.

impl Solution {
    #[allow(clippy::cast_precision_loss)]
    pub fn minimum_boxes(n: i32) -> i32 {
        let mut n = u64::from(n as u32);
        let mut h = ((n * 6) as f64).cbrt() as u64;
        let mut full_box_count = h * (h + 1) * (h + 2) / 6;

        if full_box_count > n {
            h -= 1;
            full_box_count = h * (h + 1) * (h + 2) / 6;
        }

        let base_count = h * (h + 1) / 2;

        n -= full_box_count;

        let mut extra_count = ((n * 2) as f64).sqrt() as u64;

        if extra_count * (extra_count + 1) / 2 < n {
            extra_count += 1;
        }

        (base_count + extra_count) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_boxes(n: i32) -> i32 {
        Self::minimum_boxes(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

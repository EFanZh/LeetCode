pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/discuss/490652/concise-C%2B%2B-O(N)-40ms-beats-100-(-maxmin-minmax-)>.

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut min_top = i32::MAX / 2;
        let mut max_bottom = -i32::MAX / 2;
        let mut base = 0;
        let mut extra = 0;
        let (&first, rest) = nums.split_first().unwrap();
        let last = *nums.last().unwrap();
        let mut prev = first;

        for &num in rest {
            let (bottom, top) = if num < prev { (num, prev) } else { (prev, num) };
            let diff = top - bottom;

            base += diff;
            extra = extra.max((num - first).abs().max((last - prev).abs()) - diff);
            min_top = min_top.min(top);
            max_bottom = max_bottom.max(bottom);
            prev = num;
        }

        base + extra.max((max_bottom - min_top) * 2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        Self::max_value_after_reverse(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::array;

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut map = array::from_fn::<_, 26, _>(|i| i as i32 + 1);

        chars
            .into_bytes()
            .into_iter()
            .zip(vals)
            .for_each(|(c, val)| map[usize::from(c) - usize::from(b'a')] = val);

        let mut result = 0;
        let mut min_sum = 0;
        let mut sum = 0;

        for c in s.bytes() {
            sum += map[usize::from(c) - usize::from(b'a')];
            min_sum = min_sum.min(sum);
            result = result.max((sum - min_sum).cast_unsigned());
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        Self::maximum_cost_substring(s, chars, vals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

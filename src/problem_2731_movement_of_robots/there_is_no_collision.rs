pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let mut nums = nums.into_iter().map(i64::from).collect::<Box<_>>();

        nums.iter_mut()
            .zip(s.into_bytes())
            .for_each(|(num, distance)| *num += if distance == b'L' { -i64::from(d) } else { i64::from(d) });

        nums.sort_unstable();

        let mut prev = 0;
        let mut result = 0_u64;
        let mut prev_sum = 0;

        (0..).zip(&nums).for_each(|(count, &num)| {
            prev_sum = (prev_sum + ((num - prev) as u64) * count) % MODULUS;
            prev = num;
            result += prev_sum;
            result = result.checked_sub(MODULUS).unwrap_or(result);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        Self::sum_distance(nums, s, d)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

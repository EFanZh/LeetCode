pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn add(lhs: u32, rhs: u32) -> u32 {
        let result = lhs + rhs;

        result.checked_sub(1_000_000_007).unwrap_or(result)
    }

    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low = low as u32 as usize;
        let high = high as u32 as usize;
        let zero = zero as u32 as usize;
        let one = one as u32 as usize;
        let range_size = high - low + 1;
        let capacity = (zero.max(one) + 1).max(range_size).next_power_of_two();
        let mut cache = vec![0_u32; capacity].into_boxed_slice();
        let mask = capacity - 1;

        cache[0] = 1;

        for length in 1..=high {
            cache[length & mask] = Self::add(
                cache[length.wrapping_sub(zero) & mask],
                cache[length.wrapping_sub(one) & mask],
            );
        }

        let end = (high + 1) & mask;
        let start = end.wrapping_sub(range_size) & mask;

        let (init, window) = if end < start {
            (cache[..end].iter().copied().fold(0, Self::add), &cache[start..])
        } else {
            (0, &cache[start..end])
        };

        window.iter().copied().fold(init, Self::add) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        Self::count_good_strings(low, high, zero, one)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

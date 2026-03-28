pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        let mut max_low = i32::MIN;
        let mut min_high = i32::MAX;

        original.iter().zip(bounds).for_each(|(&middle, bounds)| {
            let [low, high] = bounds.try_into().unwrap_or_default();

            max_low = max_low.max(low - middle);
            min_high = min_high.min(high - middle);
        });

        min_high.wrapping_sub(max_low).wrapping_add(1).max(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_arrays(original: Vec<i32>, bounds: Vec<Vec<i32>>) -> i32 {
        Self::count_arrays(original, bounds)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

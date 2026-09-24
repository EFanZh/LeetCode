pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut iter = arrays
            .iter()
            .map(|array| (*array.first().unwrap(), *array.last().unwrap()));

        let (mut prev_min, mut prev_max) = iter.next().unwrap();
        let mut result = 0;

        iter.for_each(|(min, max)| {
            result = result.max(prev_min.abs_diff(max).max(prev_max.abs_diff(min)));
            prev_min = prev_min.min(min);
            prev_max = prev_max.max(max);
        });

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        Self::max_distance(arrays)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

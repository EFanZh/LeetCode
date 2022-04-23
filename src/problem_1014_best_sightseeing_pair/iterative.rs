pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut iter = (0..).zip(values);
        let mut result = i32::MIN;
        let mut max = iter.next().unwrap().1;

        for (i, value) in iter {
            result = result.max(value - i + max);
            max = max.max(value + i);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        Self::max_score_sightseeing_pair(values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

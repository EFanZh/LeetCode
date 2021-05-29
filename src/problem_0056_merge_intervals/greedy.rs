pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut result = Vec::new();
        let mut iter = intervals.into_iter();

        result.push(iter.next().unwrap());

        for interval in iter {
            let previous_end = result.last_mut().unwrap().last_mut().unwrap();

            if interval[0] <= *previous_end {
                *previous_end = (*previous_end).max(interval[1]);
            } else {
                result.push(interval);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::merge(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

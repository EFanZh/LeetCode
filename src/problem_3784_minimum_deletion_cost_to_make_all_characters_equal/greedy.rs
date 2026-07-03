pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        let mut buckets = [0; 26];

        s.into_bytes()
            .into_iter()
            .zip(cost)
            .for_each(|(c, cost)| buckets[usize::from(c) - usize::from(b'a')] += u64::from(cost.cast_unsigned()));

        let mut max = 0;
        let mut sum = 0;

        for cost in buckets {
            max = max.max(cost);
            sum += cost;
        }

        (sum - max).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(s: String, cost: Vec<i32>) -> i64 {
        Self::min_cost(s, cost)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

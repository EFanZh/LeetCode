pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        _ = n;

        queries
            .into_iter()
            .map(|query| {
                let [a, b] = <[_; 2]>::map(query.try_into().ok().unwrap(), i32::cast_unsigned);
                let (a, b) = if b < a { (b, a) } else { (a, b) };
                let a_depth = a.ilog2();
                let b_depth = b.ilog2();

                let common = a
                    >> ((a ^ (b >> (b_depth - a_depth)))
                        .checked_ilog2()
                        .unwrap_or(u32::MAX)
                        .wrapping_add(1));

                let common_depth = common.ilog2();

                (a_depth + b_depth - common_depth * 2 + 1).cast_signed()
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn cycle_length_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::cycle_length_queries(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

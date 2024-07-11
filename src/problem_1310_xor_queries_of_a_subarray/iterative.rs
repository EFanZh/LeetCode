pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = arr;
        let mut acc = 0;

        for value in &mut arr {
            acc ^= *value;
            *value = acc;
        }

        queries
            .into_iter()
            .map(|query| {
                let [left, right]: [_; 2] = query.try_into().ok().unwrap();

                arr[right as usize] ^ arr.get((left as usize).wrapping_sub(1)).copied().unwrap_or(0)
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::xor_queries(arr, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

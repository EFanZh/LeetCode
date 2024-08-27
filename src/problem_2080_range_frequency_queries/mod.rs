pub mod binary_search;

pub trait RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self;
    fn query(&self, left: i32, right: i32, value: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::RangeFreqQuery;

    pub fn run<R: RangeFreqQuery>() {
        let test_cases = [(
            &[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56],
            &[((1, 2, 4), 1), ((0, 11, 33), 2)],
        )];

        for (arr, queries) in test_cases {
            let range_freq_query = R::new(arr.to_vec());

            for &((left, right, value), expected) in queries {
                assert_eq!(range_freq_query.query(left, right, value), expected);
            }
        }
    }
}

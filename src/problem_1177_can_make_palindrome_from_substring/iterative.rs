pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let length = queries.iter().map(|query| query[1]).max().unwrap() as usize + 1;
        let mut state = 0_u32;

        let states = s
            .bytes()
            .take(length)
            .map(|c| {
                state ^= 1 << (c - b'a');

                state
            })
            .collect::<Vec<_>>();

        drop(s);

        queries
            .iter()
            .map(|query| {
                let [left, right, k]: [_; 3] = query.as_slice().try_into().unwrap();
                let (left, right, k) = (left as usize, right as usize, k as u32);
                let state = states[right] ^ states.get(left.wrapping_sub(1)).copied().unwrap_or(0);

                state.count_ones() / 2 <= k
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        Self::can_make_pali_queries(s, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;

        envelopes.sort_unstable_by_key(|envelope| {
            let [w, h]: [_; 2] = envelope.as_slice().try_into().unwrap();

            (w, Reverse(h))
        });

        let mut cache = Vec::with_capacity(envelopes.len());

        for height in envelopes.into_iter().map(|envelope| envelope[1]) {
            if let Err(index) = cache.binary_search(&height) {
                if let Some(value) = cache.get_mut(index) {
                    *value = height;
                } else {
                    cache.push(height);
                }
            }
        }

        cache.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        Self::max_envelopes(envelopes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn dfs(state: u32, mut iter: Iter<u32>) -> u32 {
        if let Some(&word) = iter.next() {
            let mut result = Self::dfs(state, iter.clone());

            if state & word == 0 {
                result = result.max(Self::dfs(state | word, iter));
            }

            result
        } else {
            state.count_ones()
        }
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let words = arr
            .into_iter()
            .filter_map(|word| {
                let mut state = 0;

                for c in word.into_bytes() {
                    let probe = 1 << (c - b'a');

                    if state & probe == 0 {
                        state |= probe;
                    } else {
                        return None;
                    }
                }

                Some(state)
            })
            .collect::<Vec<_>>();

        Self::dfs(0, words.iter()) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_length(arr: Vec<String>) -> i32 {
        Self::max_length(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::mem;

impl Solution {
    fn dfs(word: &mut [u8], cache: &mut HashMap<&[u8], u32>) -> u32 {
        #[expect(clippy::option_if_let_else, reason = "false positive")]
        if let Some(length) = cache.get_mut(word) {
            if *length == 0 {
                if word.len() < 2 {
                    *length = 1;

                    *length
                } else {
                    let (extra, rest) = word.split_first_mut().unwrap();

                    let mut result = Self::dfs(rest, cache);

                    for i in 0..rest.len() {
                        mem::swap(extra, &mut rest[i]);

                        result = result.max(Self::dfs(rest, cache));
                    }

                    word.rotate_left(1);

                    result += 1;

                    *cache.get_mut(word).unwrap() = result;

                    result
                }
            } else {
                *length
            }
        } else {
            0
        }
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut cache = words.iter().map(|word| (word.as_bytes(), 0)).collect::<HashMap<_, _>>();
        let mut result = 0;
        let mut buffer = Vec::new();

        for word in words.iter().map(String::as_bytes) {
            buffer.extend(word);

            result = result.max(Self::dfs(&mut buffer, &mut cache));

            buffer.clear();
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_str_chain(words: Vec<String>) -> i32 {
        Self::longest_str_chain(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

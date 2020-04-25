pub struct Solution {}

use std::mem;

impl Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (text1, text2) = if text2.len() < text1.len() {
            (text2.into_bytes(), text1.into_bytes())
        } else {
            (text1.into_bytes(), text2.into_bytes())
        };

        let mut cache = vec![0; text1.len() + 1];

        for y_j in text2 {
            let mut old_value = 0;

            for (i, x_i) in text1.iter().enumerate() {
                let new_value = if *x_i == y_j {
                    old_value + 1
                } else {
                    cache[i].max(cache[i + 1])
                };

                old_value = mem::replace(&mut cache[i + 1], new_value);
            }
        }

        *cache.last().unwrap()
    }
}

impl super::Solution for Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::longest_common_subsequence(text1, text2)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::ptr::NonNull;

type CacheKey = (NonNull<[u8]>, NonNull<[u8]>);

impl Solution {
    fn is_scramble_helper(s1: &[u8], s2: &[u8], cache: &mut HashMap<CacheKey, bool>) -> bool {
        if s1.len() < 2 {
            s1 == s2
        } else {
            let key = (NonNull::from(s1), NonNull::from(s2));

            if let Some(&result) = cache.get(&key) {
                result
            } else {
                let mut count = [0; 256];

                for (c1, c2) in s1.iter().zip(s2) {
                    if c1 != c2 {
                        count[usize::from(*c1)] += 1;
                        count[usize::from(*c2)] -= 1;
                    }
                }

                let result = if count.iter().all(|x| *x == 0) {
                    let mut i = 1;

                    loop {
                        let (s1_left, s1_right) = s1.split_at(i);
                        let (s2_left_1, s2_right_1) = s2.split_at(i);

                        if Self::is_scramble_helper(s1_left, s2_left_1, cache)
                            && Self::is_scramble_helper(s1_right, s2_right_1, cache)
                        {
                            break true;
                        }

                        let (s2_left_2, s2_right_2) = s2.split_at(s1.len() - i);

                        if Self::is_scramble_helper(s1_left, s2_right_2, cache)
                            && Self::is_scramble_helper(s1_right, s2_left_2, cache)
                        {
                            break true;
                        }

                        i += 1;

                        if i == s1.len() {
                            break false;
                        }
                    }
                } else {
                    false
                };

                cache.insert(key, result);

                result
            }
        }
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::is_scramble_helper(s1.as_bytes(), s2.as_bytes(), &mut HashMap::new())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_scramble(s1: String, s2: String) -> bool {
        Self::is_scramble(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

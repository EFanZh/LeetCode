pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::convert::TryInto;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let languages = languages
            .into_iter()
            .map(|languages| {
                languages
                    .into_iter()
                    .map(|language| language as u16 - 1)
                    .collect::<HashSet<_>>()
            })
            .collect::<Box<_>>();

        let mut need_to_learn = vec![false; languages.len()].into_boxed_slice();
        let mut language_counts = vec![0_u16; n as u32 as usize].into_boxed_slice();
        let mut total = 0;
        let mut max_common_languages = 0;

        for friendship in friendships {
            let [x, y]: [_; 2] = friendship.try_into().ok().unwrap();
            let x = x as u32 as usize - 1;
            let y = y as u32 as usize - 1;
            let x_languages = &languages[x];
            let y_languages = &languages[y];

            if x_languages.intersection(y_languages).next().is_none() {
                for (i, languages) in [(x, x_languages), (y, y_languages)] {
                    if let state @ false = &mut need_to_learn[i] {
                        *state = true;

                        total += 1;

                        for &language in languages {
                            let count = &mut language_counts[usize::from(language)];

                            *count += 1;

                            max_common_languages = max_common_languages.max(*count);
                        }
                    }
                }
            }
        }

        i32::from(total - max_common_languages)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        Self::minimum_teachings(n, languages, friendships)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

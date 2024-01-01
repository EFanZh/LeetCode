pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

struct BitSet {
    data: [u64; 8],
}

impl BitSet {
    fn new(data: Vec<i32>) -> Self {
        let mut inner_data = [0_u64; 8];

        for x in data {
            let x = x as u32;
            let index = x / 64;
            let bit = x % 64;

            inner_data[index as usize] |= 1 << bit;
        }

        Self { data: inner_data }
    }

    fn has_common_bit(&self, other: &Self) -> bool {
        self.data.iter().zip(&other.data).any(|(lhs, rhs)| (lhs & rhs) != 0)
    }

    fn bits(&self, mut f: impl FnMut(u32)) {
        let mut start = 0;

        for &bits in &self.data {
            let mut bits = bits;

            while bits != 0 {
                f(start + bits.trailing_zeros());

                bits &= bits - 1;
            }

            start += 64;
        }
    }
}

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let languages = languages.into_iter().map(BitSet::new).collect::<Box<_>>();

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

            if !x_languages.has_common_bit(y_languages) {
                for (i, languages) in [(x, x_languages), (y, y_languages)] {
                    if let state @ false = &mut need_to_learn[i] {
                        *state = true;

                        total += 1;

                        languages.bits(|language| {
                            let count = &mut language_counts[language as usize - 1];

                            *count += 1;

                            max_common_languages = max_common_languages.max(*count);
                        });
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

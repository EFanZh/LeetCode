pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = [0; 26];

        for (i, c) in (0_u8..).zip(order.bytes()) {
            map[usize::from(c) - usize::from(b'a')] = i;
        }

        let mut iter = words.iter().map(String::as_str);

        iter.next().map_or(true, |mut prev| {
            let key_fn = |c| map[usize::from(c) - usize::from(b'a')];

            for word in iter {
                if prev.bytes().map(key_fn).le(word.bytes().map(key_fn)) {
                    prev = word;
                } else {
                    return false;
                }
            }

            true
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        Self::is_alien_sorted(words, order)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

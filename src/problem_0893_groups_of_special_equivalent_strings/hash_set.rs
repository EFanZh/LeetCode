pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for word in &words {
            let word = word.as_bytes();
            let mut key = [0_u8; 26];

            for &c in word.iter().step_by(2) {
                key[usize::from(c) - usize::from(b'a')] += 1;
            }

            for &c in word[1..].iter().step_by(2) {
                key[usize::from(c) - usize::from(b'a')] += 16;
            }

            set.insert(key);
        }

        set.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        Self::num_special_equiv_groups(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut index = letters
            .binary_search_by(|&c| if c <= target { Ordering::Less } else { Ordering::Greater })
            .unwrap_err();

        if index == letters.len() {
            index = 0;
        }

        letters[index]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        Self::next_greatest_letter(letters, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

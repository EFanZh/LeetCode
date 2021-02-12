pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut available = [0_usize; 26];

        for letter in magazine.bytes() {
            available[usize::from(letter - b'a')] += 1;
        }

        for letter in ransom_note.bytes() {
            let count = &mut available[usize::from(letter - b'a')];

            if *count == 0 {
                return false;
            }

            *count -= 1;
        }

        true
    }
}

impl super::Solution for Solution {
    fn can_construct(ransom_note: String, magazine: String) -> bool {
        Self::can_construct(ransom_note, magazine)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

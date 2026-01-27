pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

#[derive(Default)]
struct Vowels {
    a: u32,
    e: u32,
    i: u32,
    o: u32,
    u: u32,
}

impl Vowels {
    fn get_mut(&mut self, c: u8) -> Option<&mut u32> {
        match c {
            b'a' => Some(&mut self.a),
            b'e' => Some(&mut self.e),
            b'i' => Some(&mut self.i),
            b'o' => Some(&mut self.o),
            b'u' => Some(&mut self.u),
            _ => None,
        }
    }

    fn is_complete(&self) -> bool {
        self.a != 0 && self.e != 0 && self.i != 0 && self.o != 0 && self.u != 0
    }
}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.into_bytes();
        let k = k.cast_unsigned();
        let mut result = 0;
        let mut vowels_end = 0;
        let mut vowels = Vowels::default();
        let mut consonants_start = 0;
        let mut consonants_start_count = 0;
        let mut consonants_end = 0;
        let mut consonants_end_count = 0;

        for &c in &word {
            if let Some(count) = vowels.get_mut(c) {
                *count += 1;
            } else {
                consonants_start_count += 1;
                consonants_end_count += 1;
            }

            while vowels.is_complete() {
                if let Some(count) = vowels.get_mut(word[vowels_end]) {
                    *count -= 1;
                }

                vowels_end += 1;
            }

            while consonants_start_count > k {
                consonants_start_count -= u32::from(vowels.get_mut(word[consonants_start]).is_none());
                consonants_start += 1;
            }

            while consonants_end_count >= k {
                if let Some(&old) = word.get(consonants_end) {
                    consonants_end_count -= u32::from(vowels.get_mut(old).is_none());
                    consonants_end += 1;
                } else {
                    break;
                }
            }

            result += vowels_end.min(consonants_end).saturating_sub(consonants_start) as u64;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_of_substrings(word: String, k: i32) -> i64 {
        Self::count_of_substrings(word, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

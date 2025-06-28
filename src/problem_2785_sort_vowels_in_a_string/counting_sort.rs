pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut vowels = [0_u32; 10];

        for &c in &s {
            let index = match c {
                b'A' => 0,
                b'E' => 1,
                b'I' => 2,
                b'O' => 3,
                b'U' => 4,
                b'a' => 5,
                b'e' => 6,
                b'i' => 7,
                b'o' => 8,
                b'u' => 9,
                _ => continue,
            };

            vowels[index] += 1;
        }

        let mut index = 0;

        for c in &mut s {
            if matches!(c, b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u') {
                while vowels[index] == 0 {
                    index += 1;
                }

                *c = b"AEIOUaeiou"[index];

                vowels[index] -= 1;
            }
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_vowels(s: String) -> String {
        Self::sort_vowels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

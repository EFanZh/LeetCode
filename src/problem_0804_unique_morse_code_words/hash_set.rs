pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::io::Write;

        const MORSE_CODES: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---",
            ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
        ];

        let mut set = HashSet::<Box<[u8]>>::new();
        let mut buffer = [0; 100];

        for word in &words {
            let mut writer: &mut [u8] = &mut buffer;

            for c in word.bytes() {
                writer
                    .write_all(MORSE_CODES[usize::from(c) - usize::from(b'a')].as_bytes())
                    .unwrap();
            }

            let remaining_length = writer.len();
            let code = &buffer[..buffer.len() - remaining_length];

            if !set.contains(code) {
                set.insert(code.to_vec().into_boxed_slice());
            }
        }

        set.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_morse_representations(words: Vec<String>) -> i32 {
        Self::unique_morse_representations(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

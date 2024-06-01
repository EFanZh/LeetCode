pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let word = word.as_bytes();
        let mut iter = word.iter().copied().enumerate();
        let mut result = 0;

        'outer: while let Some((start, c)) = iter.next() {
            let index = match c {
                b'a' => 0,
                b'e' => 1,
                b'i' => 2,
                b'o' => 3,
                b'u' => 4,
                _ => continue,
            };

            let mut counts = [0_u8; 5];

            counts[index] = 1;

            let mut required = 4;

            // Not enough vowels.

            loop {
                if let Some((_, c)) = iter.next() {
                    let index = match c {
                        b'a' => 0,
                        b'e' => 1,
                        b'i' => 2,
                        b'o' => 3,
                        b'u' => 4,
                        _ => continue 'outer,
                    };

                    let count = &mut counts[index];

                    if *count == 0 {
                        required -= 1;
                    }

                    *count += 1;

                    if required == 0 {
                        break;
                    }
                } else {
                    break 'outer;
                }
            }

            // Has enough vowels.

            let mut i = start;

            loop {
                loop {
                    let index = match word[i] {
                        b'a' => 0,
                        b'e' => 1,
                        b'i' => 2,
                        b'o' => 3,
                        _ => 4,
                    };

                    i += 1;

                    let count = &mut counts[index];

                    *count -= 1;

                    if *count == 0 {
                        break;
                    }
                }

                let length = i - start;

                loop {
                    result += length;

                    if let Some((_, c)) = iter.next() {
                        let index = match c {
                            b'a' => 0,
                            b'e' => 1,
                            b'i' => 2,
                            b'o' => 3,
                            b'u' => 4,
                            _ => continue 'outer,
                        };

                        let count = &mut counts[index];

                        *count += 1;

                        if *count == 1 {
                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_vowel_substrings(word: String) -> i32 {
        Self::count_vowel_substrings(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

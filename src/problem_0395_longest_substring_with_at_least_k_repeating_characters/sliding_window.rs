pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_unique_characters(s: &[u8]) -> i32 {
        let mut counts = [0; 26];

        for c in s {
            counts[usize::from(c - b'a')] += 1;
        }

        counts.iter().filter(|&&x| x != 0).count() as _
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let mut result = 0;

        for max_unique_chars in 1..=Self::count_unique_characters(&s) {
            let mut counts = [0; 26];
            let mut start = 0;
            let mut end = 0;
            let mut unique_chars = 0;
            let mut frequent_chars = 0;

            // Shortest leftmost window with `max_unique_chars` unique characters.

            loop {
                let count = &mut counts[usize::from(s[end] - b'a')];

                end += 1;
                *count += 1;

                if *count == k {
                    frequent_chars += 1;
                }

                if *count == 1 {
                    unique_chars += 1;

                    if unique_chars == max_unique_chars {
                        break;
                    }
                }
            }

            // Slide the window while keeping `max_unique_chars` unique characters with the window.

            loop {
                if frequent_chars == unique_chars {
                    result = result.max(end - start);
                }

                if let Some(c) = s.get(end) {
                    let count = &mut counts[usize::from(c - b'a')];

                    end += 1;
                    *count += 1;

                    if *count == k {
                        frequent_chars += 1;
                    }

                    if *count == 1 {
                        // Got a new character, need to shrink.

                        loop {
                            let count_2 = &mut counts[usize::from(s[start] - b'a')];

                            start += 1;

                            if *count_2 == k {
                                frequent_chars -= 1;
                            }

                            *count_2 -= 1;

                            if *count_2 == 0 {
                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_substring(s: String, k: i32) -> i32 {
        Self::longest_substring(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let s = s.into_bytes();
        let mut n = s.len() as u16;
        let mut k = k as u16;
        let letter = letter as u8;
        let mut repetition = repetition as u16;
        let mut remaining_letters = s.iter().fold(0, |count, &c| count + u16::from(c == letter));
        let mut stack = Vec::with_capacity(usize::from(k));

        for c in s {
            if c == letter {
                while let Some(&top) = stack.last() {
                    if c < top && k < n {
                        stack.pop();
                        k += 1;
                    } else {
                        break;
                    }
                }

                remaining_letters -= 1;

                if k != 0 {
                    stack.push(c);
                    k -= 1;
                    repetition = repetition.wrapping_sub(1);
                }
            } else {
                if c < letter {
                    while let Some(&top) = stack.last() {
                        if c < top && k < n {
                            if top == letter {
                                if repetition == remaining_letters {
                                    break;
                                }

                                repetition = repetition.wrapping_add(1);
                            }

                            stack.pop();
                            k += 1;
                        } else {
                            break;
                        }
                    }
                } else {
                    while let Some(&top) = stack.last() {
                        if c < top && k < n {
                            stack.pop();
                            k += 1;
                        } else {
                            break;
                        }
                    }
                }

                if k != 0 && repetition != k {
                    stack.push(c);
                    k -= 1;
                }
            }

            n -= 1;
        }

        String::from_utf8(stack).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        Self::smallest_subsequence(s, k, letter, repetition)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

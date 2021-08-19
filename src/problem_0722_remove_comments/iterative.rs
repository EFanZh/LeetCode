pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = String::new();
        let mut in_block_comment = false;

        'next_line: for line in source {
            let mut i = if in_block_comment {
                if let Some(i) = line.find("*/") {
                    in_block_comment = false;

                    i + 2
                } else {
                    continue 'next_line;
                }
            } else {
                0
            };

            loop {
                if let Some((j, is_block)) = line.get(i + 1..).and_then(|right| {
                    (i..)
                        .zip(line[i..].bytes().zip(right.bytes()))
                        .find_map(|(i, t)| match t {
                            (b'/', b'/') => Some((i, false)),
                            (b'/', b'*') => Some((i, true)),
                            _ => None,
                        })
                }) {
                    current_line.push_str(&line[i..j]);

                    if is_block {
                        if let Some(k) = line[j + 2..].find("*/") {
                            i = j + k + 4;
                        } else {
                            in_block_comment = true;

                            continue 'next_line;
                        }
                    } else {
                        break;
                    }
                } else {
                    current_line.push_str(&line[i..]);

                    break;
                }
            }

            if !current_line.is_empty() {
                result.push(current_line);

                current_line = line;
                current_line.clear();
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_comments(source: Vec<String>) -> Vec<String> {
        Self::remove_comments(source)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}

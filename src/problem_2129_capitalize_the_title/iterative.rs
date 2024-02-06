pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut title = title.into_bytes();
        let mut iter = title.iter_mut();

        'outer: while let Some(c1) = iter.next() {
            if let Some(c2) = iter.next() {
                if *c2 == b' ' {
                    c1.make_ascii_lowercase();
                } else {
                    c2.make_ascii_lowercase();

                    if let Some(c3) = iter.next() {
                        if *c3 == b' ' {
                            c1.make_ascii_lowercase();
                        } else {
                            c1.make_ascii_uppercase();
                            c3.make_ascii_lowercase();

                            loop {
                                if let Some(c) = iter.next() {
                                    if *c == b' ' {
                                        break;
                                    }

                                    c.make_ascii_lowercase();
                                } else {
                                    break 'outer;
                                }
                            }
                        }
                    } else {
                        c1.make_ascii_lowercase();

                        break;
                    }
                }
            } else {
                c1.make_ascii_lowercase();

                break;
            }
        }

        String::from_utf8(title).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn capitalize_title(title: String) -> String {
        Self::capitalize_title(title)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
